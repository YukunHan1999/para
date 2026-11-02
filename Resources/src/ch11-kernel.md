# kernel

```bash
cat /proc/version          # 内核版本
cat /proc/cmdline          # 启动参数
cat /proc/cpuinfo          # CPU 信息
cat /proc/meminfo          # 内存信息
cat /proc/kallsyms | head  # 内核符号表（函数地址）
dmesg | head -50           # 内核启动日志
ls /sys/kernel/            # sysfs 下内核对象
cat /proc/interrupts       # 中断统计
cat /proc/slabinfo         # slab 分配器状态
```

```bash
insmod hello.ko
lsmod
dmesg | tail              # 看模块的 printk 输出
rmmod hello
```

```bash
b start_kernel
b do_sys_open
b schedule
c
```

```bash
mount -t tracefs none /sys/kernel/tracing

cd /sys/kernel/tracing
echo function > current_tracer
echo 1 > tracing_on
# 执行一些操作
echo 0 > tracing_on
cat trace | head -50
```

先熟悉环境：在 shell 里把上面 /proc、/sys、dmesg 的命令跑一遍，知道内核暴露了哪些信息。
写一个 hello 模块：在容器里编译，送进 QEMU 加载，看 printk 输出。这是最小的“改内核代码并验证”的练习。
用 GDB 跟一次系统调用：比如 b do_sys_open，在 shell 里 cat /proc/version，看断点命中、看调用栈。理解从用户态到内核态的路径。
读 init/main.c 的 start_kernel：对照 dmesg 的启动日志，看每个初始化阶段对应哪行代码。这是理解内核启动流程的入口。
找一个 staging 或文档的小问题改：按之前说的 checkpatch.pl → 编译 → QEMU 验证 → git format-patch 的流程走一遍，熟悉补丁流程。


```bash
容器运行异构cpu架构 x86_64, aarch64, arm64, riscv64

binfmt_misc: 在内核层面进行模式匹配，当系统尝试执行一个文件时，如果标准格式（如 ELF）无法处理，内核会查询 binfmt_misc 的注册表，看是否有规则能识别它。
    注册: 用户通过向 /proc/sys/fs/binfmt_misc/register 文件写入特定格式的字符串来添加规则
    触发: 当一个文件被执行时，内核的 search_binary_handler() 函数会遍历规则。匹配成功后内核就会调用注册时指定的解释器, 并将要执行的文件作为参数传递给它。

fix-binary: 
    问题: binfmt_misc 的默认行为是“惰性打开”。这意味着，只有当内核真正遇到一个匹配的文件时，它才会根据解释器的路径去尝试打开它。如果解释器（如 qemu-aarch64-static）安装在宿主机上，而容器内没有这个文件，内核在容器的根文件系统（rootfs）中查找解释器时就会失败，导致“no such file or directory”错误?
    处理: 注册时即打开。当带有 F 标志的规则被注册时，内核会立即打开（open）指定的解释器文件，并持续保持这个文件描述符（file descriptor）处于打开状态，直到该规则被移除。执行时：直接使用已打开的句柄。当后续有文件触发该规则时，内核不再去文件系统中查找解释器，而是直接 exec 之前已经保持打开的那个文件描述符。

    binfmt_misc 是一个内核级的格式匹配与路由框架。而 fix-binary 标志通过将解释器的打开时机提前到规则注册阶段，并长期持有其文件描述符，巧妙地解决了在命名空间隔离（尤其是容器）环境下，解释器路径无法被正确解析的根本性矛盾。这使得宿主机上安装的 QEMU 静态模拟器能够无缝地为任何架构的容器提供服务。

宿主机支持:
    sudo pacman -S qemu-user-static qemu-user-static-binfmt
    sudo systemctl restart systemd-binfmt

验证:
    docker run -d --platform linux/arm64 arm64v8/alpine sleep 300
    docker exec -it <cid> sh
    ps aux
    docker stop <cid>
    ps -eo pid,comm,args | grep '[q]emu'

总结:
    如果想在容器中运行异构的程序需要借助binfmt-misc+fix-binary的机制, 宿主机提供qemu-user解释器.
    运行异构的系统,需要容器中启动qemu-system-* 自己配置启动参数, 如果需要kvm能力, 需要容器启动时挂在--device /dev/kvm, 容器在这里只提供隔离环境.

    换角度思考透明执行异构程序 (binfmt-misc): 
        1. 容器中运行异构程序, 需要fix-binary
        2. 跨架构 chroot 环境, 不需要容器, 需要fix-binary
        3. 宿主机上直接透明运行异构程序或者其他格式的文件透明执行, 不需要fix-binary
```


```bash
rootfs的制作:
    极简制作: 不带glibc库,只能运行静态编译的文件
        mkdir -p /workspace/rootfs/simple-rootfs/bin \
                /workspace/rootfs/simple-rootfs/dev \
                /workspace/rootfs/simple-rootfs/proc \
                /workspace/rootfs/simple-rootfs/sys
        # 加载模块
        cp /usr/bin/busybox /workspace/rootfs/simple-rootfs/bin
        ln -sf busybox /workspace/rootfs/simple-rootfs/bin/sh
        ln -sf busybox /workspace/rootfs/simple-rootfs/bin/mount

    带glibc制作:
        最直接-pacstrap制作:
            sudo pacman -S arch-install-scripts
            sudo mkdir -p /workspace/dev/kernel_dev/workspace/rootfs/pacstrap-rootfs
            sudo pacstrap -C /etc/pacman.conf /workspace/dev/kernel_dev/workspace/rootfs/pacstrap-rootfs base
        最轻量-bootstrap tarball(容器上也可以制作):
            curl -L -O https://mirrors.edge.kernel.org/archlinux/iso/latest/archlinux-bootstrap-x86_64.tar.zst --create-dirs --output-dir /tmp
            sudo mkdir -p /workspace/rootfs/bootstrap-rootfs
            sudo tar -xaf /tmp/archlinux-bootstrap-*.tar.zst -C /workspace/rootfs/bootstrap-rootfs --strip-components=1
        面向构建-mkarchroot:
            sudo pacman -S devtools
            sudo mkarchroot /workspace/dev/kernel_dev/workspace/rootfs/mkarchroot-rootfs base

        制造好后记得修改密码:
            # sudo passwd --root /workspace/dev/kernel_dev/workspace/rootfs/mkarchroot-rootfs root
            sudo arch-chroot /workspace/rootfs/pacstrap-rootfs passwd

        
    打包(避免重复打包构建overlyfs, 解决分层持久化的问题):
        # 创建一个软盘
        qemu-img create -f qcow2 persistent-overlay.qcow2 2G
        # 在宿主机上直接完成格式化
            # 1. 加载 nbd 内核模块
            sudo modprobe nbd max_part=8
            # 2. 把 qcow2 镜像连接到 /dev/nbd0
            sudo qemu-nbd --connect=/dev/nbd0 --format=qcow2 persistent-overlay.qcow2
            # 3. 格式化
            sudo mkfs.ext4 /dev/nbd0
            # 4. 断开连接
            sudo qemu-nbd --disconnect /dev/nbd0

        printf '%s\n' \
            '#!/bin/sh' \
            '/bin/mount -t proc none /proc' \
            '/bin/mount -t sysfs none /sys' \
            '/bin/mount -t devtmpfs none /dev || true' \
            'mkdir -p /mnt/overlay-disk' \
            '/bin/mount /dev/vda /mnt/overlay-disk' \
            'mkdir -p /mnt/overlay-disk/upper' \
            'mkdir -p /mnt/overlay-disk/work' \
            'mkdir -p /mnt/newroot' \
            '/bin/mount -t overlay overlay -o lowerdir=/,upperdir=/mnt/overlay-disk/upper,workdir=/mnt/overlay-disk/work /mnt/newroot' \
            'echo "Hello from kernel"' \
            'exec /bin/switch_root /mnt/newroot /sbin/init' \
            | sudo tee /workspace/rootfs/simple-rootfs/init
        chmod +x /workspace/rootfs/simple-rootfs/init
        cd /workspace/rootfs/simple-rootfs && find . -print-1 | cpio --null -ov --format=newc | gzip -9 > /workspace/build/simple-rootfs.cpio.gz 

    共享目录(解决外部和内部通信的问题):
        离线方式(宿主机上处理,容器内不行):
            sudo pacman -S libguestfs
            guestfish --rw -a /workspace/dev/kernel_dev/workspace/rootfs/persistent-overlay.qcow2
            run
            list-filesystems
            mount /dev/sda /
            copy-in /workspace/dev/rust_dev/workspace/test/demo /upper/root/
            exit

        在线方式:
            # qemu启动参数添加
		    -virtfs local,path=/workspace/app,mount_tag=hostshare,security_model=none \
            # 启动后
            mkdir -p /mnt/share
            /bin/mount -t 9p -o trans=virtio hostshare /mnt/share
```
