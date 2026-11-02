# CGROUP

你已经有源码和编译能力，这让你可以跳过单纯看文档的阶段，直接从**可观测的行为**和**可修改的代码**两端夹击。cgroup v2 的心智模型可以压缩成三件事：**一棵树、每个节点挂着一组资源旋钮、进程挂在节点上**。下面的实验设计就是围绕这个模型，从外部行为逐步深入到内核源码内部。

---

```bash
# 查询是否挂载cgroup
mount | grep cgroup2
stat -fc %T /sys/fs/cgroup/
cat /proc/filesystems | grep cgroup2

# 手动挂载 cgroup2, none 表示没有设备,内容由内核提供; 
# cgroup 在内核里面维护的是: 一组"进程分组" 的层级树 + 每组上挂载的控制器(cpu, memory, pids...)
mount -t cgroup2 none /sys/fs/cgroup
```


### 实验一：把 cgroup v2 文件系统“看穿”

**目的**：建立对“树 + 旋钮 + 进程归属”的直观感受，不碰代码。

**操作**：

1. **确认挂载点**：`mount | grep cgroup2`，确认 `/sys/fs/cgroup` 是唯一层级。
2. **观察根节点**：
   ```bash
   ls /sys/fs/cgroup/
   cat /sys/fs/cgroup/cgroup.controllers
   cat /sys/fs/cgroup/cgroup.subtree_control
   ```
   `cgroup.controllers` 列出当前系统所有可用的控制器（cpu、memory、pids、io 等）；`cgroup.subtree_control` 显示哪些控制器被下放给了子节点。

3. **创建一个空 cgroup**：
   ```bash
   mkdir /sys/fs/cgroup/lab
   ls /sys/fs/cgroup/lab/
   ```
   你会看到 `cgroup.controllers`、`cgroup.subtree_control`、`cgroup.procs` 等核心文件，以及该节点**已启用**的控制器专属文件（如 `pids.current`、`pids.max`）。

4. **观察进程归属**：
   ```bash
   cat /proc/self/cgroup
   ```
   输出类似 `0::/user.slice/...`，这是当前 shell 所在的 cgroup 路径。**每个进程有且仅有一个 cgroup 归属**，由 `task_struct` 中的 `css_set` 指针最终指向。

**源码锚点**：`kernel/cgroup/cgroup.c` 中的 `cgroup_init()` 负责在启动时初始化根 cgroup 和所有控制器。读它的开头，你会看到 `init_css_set` 这个全局变量被加入哈希表，这是所有进程初始归属的起点。

---

### 实验二：用 pids 控制器“触碰”限制的边界

**目的**：选一个逻辑最简单、源码路径最短的控制器，完整走通“写旋钮 → 内核记账 → 触发限制 → 观察返回码”的闭环。**pids 控制器是理想的起点**，它不涉及复杂的资源回收或调度，核心逻辑清晰。

**操作**：

1. 在 `/sys/fs/cgroup/lab` 下写入：
   ```bash
   echo 5 | tee /sys/fs/cgroup/lab/pids.max
   ```

2. 把自己的 shell 移入该 cgroup：
   ```bash
   echo $$ | tee /sys/fs/cgroup/lab/cgroup.procs
   ```

3. 尝试 fork：
   ```bash
    for i in $(seq 1 10); do sleep 100 & done
   ```
   你会看到在某个时刻 `fork` 失败，返回 `Resource temporarily unavailable`（`EAGAIN`）。这是因为 `pids.current` 触及了 `pids.max`，`fork()`/`clone()` 被内核拦截。

4. 观察：
   ```bash
   cat /sys/fs/cgroup/lab/pids.current
   cat /sys/fs/cgroup/lab/pids.events   # 看 max 事件计数
   ```

**源码锚点**：`kernel/cgroup/pids.c`。搜索 `pids_try_charge` 和 `pids_can_fork`，你会看到 `pids.max` 的写入如何设置 `pids->max`，以及每次 `fork` 时 `pids_try_charge()` 如何原子地增加计数并在超限时返回 `-EAGAIN`。这个文件通常只有几百行，非常适合作为第一个通读的控制器源码。

---

### 实验三：让 memory 控制器“杀死”一个进程

**目的**：观察 cgroup 的**资源回收与 OOM 机制**。memory 控制器比 pids 深得多，但它能让你看到 cgroup 如何与内核的内存管理子系统（memcg）深度耦合。

**操作**：

1. 创建新的 cgroup（或复用 lab）：
   ```bash
   sudo mkdir /sys/fs/cgroup/memlab
   echo 50M | sudo tee /sys/fs/cgroup/memlab/memory.max
   echo $$ | sudo tee /sys/fs/cgroup/memlab/cgroup.procs
   ```

2. 在另一个终端监控：
   ```bash
   watch -n 0.5 'cat /sys/fs/cgroup/memlab/memory.current'
   ```

3. 在本 shell 中分配内存：
   ```bash
   python3 -c "a = bytearray(100*1024*1024)"
   ```
   你应该会看到进程被 **OOM killer** 终止，或分配失败。`memory.current` 在触及 `memory.max` 后会触发内存回收（reclaim），回收不足则调用 OOM killer。

4. 观察 OOM 事件：
   ```bash
   cat /sys/fs/cgroup/memlab/memory.events
   ```
   `oom` 和 `oom_kill` 计数会增加。如果设置 `memory.oom.group=1`，整个 cgroup 的进程会被一起杀死。

**源码锚点**：`mm/memcontrol.c`。这个文件巨大，不要试图通读。聚焦搜索 `memory_max_write` 和 `try_charge`。`memory.max` 的写入最终设置 `memcg->memory.max`；而每次内存分配（page fault）路径上的 `try_charge()` 会在用量超过 `memory.max` 时触发 `mem_cgroup_oom()`。这就是“限制 → 记账 → 回收 → OOM”的完整链条在内核中的落点。

---

### 实验四：追踪“进程 → css_set → cgroup”的关联路径

**目的**：cgroup v2 的核心数据结构关联，是理解一切控制器如何生效的钥匙。你的目标是搞清楚：**当把 `$$` 写入 `cgroup.procs` 时，内核到底改了哪些指针**。

**源码阅读路径**：

1. **入口**：`kernel/cgroup/cgroup.c` 中的 `cgroup_procs_write()` → `__cgroup_procs_write()`。这是 `echo $$ > cgroup.procs` 的内核入口。
2. **关键函数**：`cgroup_migrate()` / `cgroup_migrate_execute()`。进程迁移的核心不是直接改 `task_struct->cgroups`，而是通过 **`css_set`** 这个中间层。
3. **数据结构**：
   - `struct css_set`：一个**不可变的快照**，持有指向一组 `cgroup_subsys_state` 的指针数组 `subsys[]`。相同归属的进程**共享**同一个 `css_set`。
   - `task_struct->cgroups` 是一个 `css_set` 指针。
   - `cgroup_subsys_state`（css）是**每个控制器在每个 cgroup 上的实例状态**。`memcg`、`cpu_cgroup` 等都是 css 的嵌入或派生。

4. **验证**：用 `drgn` 或 `crash`  attach 到运行中的内核，对当前 shell 的 `task_struct` 打印 `task->cgroups`，再打印 `css_set->subsys[]` 中每个 css 的 `cgroup` 指针，与 `/proc/self/cgroup` 的路径对照。

**心智模型**：`task_struct` → `css_set` → `cgroup_subsys_state` → `cgroup`。`css_set` 是**共享的去重层**：内核维护一个哈希表，相同 cgroup 组合的进程复用同一个 `css_set`，避免每个进程都持有一份冗余的控制器状态。

---

### 实验五：编译一个“会说话”的内核

**目的**：利用你的编译能力，在内核源码中插入**观测点**，让 cgroup 的关键路径主动“告诉你”它在做什么。这比被动读代码有效得多。

**选项 A（最小侵入）**：在 `pids_try_charge` 或 `mem_cgroup_try_charge` 的开头加一行 `pr_info`，打印当前 `task->pid`、`css->cgroup->kn->name`（cgroup 名称）和当前计数。编译、安装、启动，然后在实验二或实验三中触发路径，`dmesg` 会直接显示调用顺序和参数。

**选项 B（观察初始化）**：在 `cgroup_init()` 的循环中，对每个 `ss->name` 打印一行。启动后 `dmesg | grep cgroup`，你会看到每个控制器在启动时被初始化的顺序，以及 `early_init` 标志的区别。

**选项 C（修改旋钮语义）**：在 `pids_can_fork()` 中把 `-EAGAIN` 改成打印一条特殊日志再返回。这让你亲身体验“控制器代码如何决定用户态看到的行为”。

**注意**：修改内核后，务必用**独立的构建目录**（`O=/path/to/build`）编译，避免污染源码树。启动新内核时保留旧内核作为回退选项。

---

### 实验六：找到 Docker 容器在 cgroup 树中的“座位”

**目的**：把前面所有抽象落到你日常使用的容器上，完成从“实验 cgroup”到“真实系统”的闭环。

**操作**：

1. 启动一个容器，让它持续运行：
   ```bash
   docker run --rm -d --name cglab --memory 100M alpine sleep 300
   ```

2. 找到容器进程的 PID 和 cgroup 路径：
   ```bash
   docker inspect --format '{{.State.Pid}}' cglab
   # 假设输出 12345
   cat /proc/12345/cgroup
   ```
   你会看到类似 `/system.slice/docker-<id>.scope` 的路径（取决于你的 cgroup 驱动配置）。

3. 进入对应的 cgroup 目录，观察 `memory.max`、`cpu.max`、`pids.max` 是否与你 `docker run` 的参数一致。`--memory 100M` 最终就落成了该目录下 `memory.max` 的值。

4. 对比**容器内 `free -m`** 与 **`memory.current`**：`free` 看到的是宿主机全局视图（受 namespace 影响有限），而 cgroup 的 `memory.current` 才是容器**真正被记账**的用量。

---

### 建议的推进顺序

| 阶段 | 实验 | 目标 | 源码焦点 |
|---|---|---|---|
| **1. 建立直觉** | 实验一 | 树、旋钮、归属 | `cgroup_init()` |
| **2. 最短闭环** | 实验二（pids） | 限制 → 记账 → 返回码 | `kernel/cgroup/pids.c` |
| **3. 深度耦合** | 实验三（memory） | 回收 → OOM → memcg | `mm/memcontrol.c` 的 `try_charge` |
| **4. 核心数据结构** | 实验四 | `task → css_set → css` | `cgroup_migrate()`、`css_set` 哈希 |
| **5. 主动观测** | 实验五 | 用 `pr_info` 让内核说话 | 任意控制器路径 |
| **6. 回到现实** | 实验六 | Docker 容器在树中的位置 | 验证 `docker run` 参数的内核落点 |

从 pids 开始，是因为它的源码路径最短、反馈最直接；memory 和 cpu 的源码要深得多，建议在实验四把 `css_set` 和 `cgroup_subsys_state` 的关系搞清楚之后再进入。你的编译能力最有价值的用法是在实验五：**不要只是读代码，让代码在运行时把关键路径打印出来**，这样内核对你来说就不再是一个黑盒了。
