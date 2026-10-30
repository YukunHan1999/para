# command

### ps查看进程

```bash
# 列出所有进程
ps aux
# 列出所有进程: PID, 状态, 阻塞内核函数, 程序名称
ps -eo pid,stat,wchan:40,comm
# 过滤可中断睡眠 S (等锁，等信号, 管道, sem_wait)
ps -eo pid,stat,wchan:40,comm | grep "S"
# 过滤不可中断睡眠 D (磁盘IO, 内核锁无法被信号唤醒)
ps -eo pid,stat,wchan:40,comm | grep "D"

# 查看进程当前阻塞内核函数
cat /proc/1234/wchan
# 打印完整内核调用栈，精确看到卡在哪一步锁操作
cat /proc/1234/stack
```

### ipcs 专门查看System V IPC量

```bash
# 查看所有System V信号量
ipcs -s
# 查看某个信号量详情 (创建PID, 最后操作PID, 等待计数)
ipcs -s -i 123
# 查看共享内存段
ipcs
```

### 内核统计 & 死锁检测

```bash
# 开启内核配置
CONFIG_LOCKDEP=y

# 打开统计
echo 1 > /proc/sys/kernel/lockdep
# 所有锁的争抢,等待,持有统计
cat /proc/lock_stat 
# 锁依赖链条，定位死锁
cat /proc/lockdep_chains
```

### 高级内核调试工具

```bash
# 跟踪futex锁阻塞事件
perf trace sleep 10

# 采样所有锁等待
perf record -g sleep 20
perf report
```

### 二进制文件查看工具

```bash
# 通用二进制查看工具
    # 基础查看
    hexdump ./bin/program
    # 单字节格式, ASCII 一并展示
    hexdump -C ./bin/program

    # 16进制查看
    xdd ./bin/program 
    # 纯二进制比特串查看(0和1)
    xdd -b ./bin/program

# 二进制可执行文件 -> 汇编代码
objdump ./bin/program
    -d 反汇编代码段(.text) 常用
    -D 程序段全部反汇编(包含数据段)量大
    -S 源码和汇编混合展示
    -T 查看符号表,找自定义函数
    -s 展示所有段十六进制原始数据
    -l 附带行号

# 查看ELF程序整体结构 readelf
    # 查看ELF头部基础信息
    readelf -h ./bin/program
    # 查看所有段(.text代码段和 .data全局变量段)
    readelf -S ./bin/program
    # 查看符号表
    readelf -s ./bin/program
```


