# Rust RDMA 编程「一、基础概览」

最近计划深入学习下 RDMA 编程，并且使用 Rust 构建一套 RDMA 通信库和 RPC 框架，同时将学习和编程的过程持久化到博客中。本篇是第一篇，也将作为系列的概览。

### 1. 基础知识

推荐以下个人认为写得很好的 RDMA 博客：

1. [RDMA 杂谈](https://zhuanlan.zhihu.com/p/164908617)
2. [RDMAmojo](https://www.rdmamojo.com)

先把 RDMA 杂谈的文章过一遍，对 RDMA 就会有一个相对明确的概念。本文就不再赘述了。

### 2. Rust RDMA 现状

目前 Rust RDMA 编程生态并不活跃，相对易上手的项目是达坦科技开源的 [async-rdma](https://github.com/datenlord/async-rdma)，它封装好了 RDMA 异步操作，非常易用。其他一些项目主要是针对 libibverbs 的封装，比如 [rdma-sys](https://github.com/datenlord/rdma-sys) 和 [rust-ibverbs](https://github.com/jonhoo/rust-ibverbs)。

### 3. 环境搭建

学习 RDMA 编程并不需要 RDMA 物理网络环境。Soft-RoCE 是 Remote Direct Memory Access (RDMA) over Converged Ethernet (RoCE) 的软件实现，可以模拟 RoCE 网卡搭建 RDMA 环境。Linux 内核中实现 Soft-RoCE 的模块名为 RXE，可以非常方便地配置一张 Soft-RoCE 网卡。

假定你使用的是 Debian 系操作系统：

```bash
# 1. 检查当前内核是否配置了 RXE 内核模块。如果是 m 或者 y 则已配置。
cat /boot/config-$(uname -r) | grep RXE
#> CONFIG_RDMA_RXE=m

# 2. 加载 RXE 内核模块
sudo modprobe ib_core
sudo modprobe rdma_ucm
sudo modprobe rdma_rxe

# 3. 查看当前网络设备。下方的以太网设备为 enp0s1，记住该名字
ip link
#> 1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
#>     link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
#> 2: enp0s1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
#>     link/ether 0a:57:a9:60:fb:79 brd ff:ff:ff:ff:ff:ff

# 4. 创建 RXE 网络设备
sudo rdma link add rxe_0 type rxe netdev enp0s1

# 5. 查看 RDMA 设备，大功告成
rdma link
#> link rxe_0/1 state ACTIVE physical_state LINK_UP netdev enp0s1
```

> 如果你的内核没有配置 RXE 模块，你可能需要自行编译 RXE 模块并装载。目前已知的 Ubuntu Cloud Image 是不装载 RXE 的，所以使用 Multipass 的 Mac 用户会麻烦点，可以考虑切换到 [UTM + Debian 12](https://mac.getutm.app/gallery/debian-12)。对于 GitHub CI 环境，笔者构建了一份[配置 Soft-RoCE 的 action 代码](https://github.com/SF-Zhou/setup-soft-roce-action)，可以直接复制到你自己的 Workflow 中使用。

配置好 RXE 设备后，还可以使用 `ib_send_bw` 工具简单压测下：

```bash
# 1. 安装 perftest，包含 ib_send_bw 工具
sudo apt install -y perftest

# 2. 启动 ib_send_bw server 端
ib_send_bw -d rxe_0
#> ************************************
#> * Waiting for client to connect... *
#> ************************************

# 3. 启动 ib_send_bw client 端，开始测试
ib_send_bw -d rxe_0 localhost
#> ---------------------------------------------------------------------------------------
#>                     Send BW Test
#>  Dual-port       : OFF          Device         : rxe_0
#>  Number of qps   : 1            Transport type : IB
#>  Connection type : RC           Using SRQ      : OFF
#>  PCIe relax order: ON
#>  ibv_wr* API     : OFF
#>  TX depth        : 128
#>  CQ Moderation   : 1
#>  Mtu             : 1024[B]
#>  Link type       : Ethernet
#>  GID index       : 1
#>  Max inline data : 0[B]
#>  rdma_cm QPs     : OFF
#>  Data ex. method : Ethernet
#> ---------------------------------------------------------------------------------------
#>  local address: LID 0000 QPN 0x0015 PSN 0x542d8e
#>  GID: 00:00:00:00:00:00:00:00:00:00:255:255:192:168:65:03
#>  remote address: LID 0000 QPN 0x0016 PSN 0xa2a3de
#>  GID: 00:00:00:00:00:00:00:00:00:00:255:255:192:168:65:03
#> ---------------------------------------------------------------------------------------
#>  #bytes     #iterations    BW peak[MB/sec]    BW average[MB/sec]   MsgRate[Mpps]
#>  65536      1000             1408.61            1036.98            0.016592
#> ---------------------------------------------------------------------------------------
```

### 4. 开始编程

万事开头难，当前已经进度斐然了。下面简单写一个 RDMA demo：

```bash
# 1. 创建 rdma-demo 项目
cargo new rdma-demo && cd rdma-demo

# 2. 增加 rdma-sys 依赖
cargo add rdma-sys

# 3. 安装 rdma-sys 所需的依赖包
sudo apt install -y libibverbs-dev librdmacm-dev

# 4. 尝试编译
cargo build
```

将 `main.rs` 修改为以下代码：

```rust
use rdma_sys::{ibv_free_device_list, ibv_get_device_list, ibv_get_device_name};

fn main() {
    let mut num_devices = 0;
    let list = unsafe { ibv_get_device_list(&mut num_devices) };
    assert!(num_devices != 0, "IB device not found!");

    let devices = unsafe { std::slice::from_raw_parts(list, num_devices as usize) };
    for device in devices {
        let name = unsafe { std::ffi::CStr::from_ptr(ibv_get_device_name(*device)) };
        println!("device name: {}", name.to_string_lossy());
    }

    unsafe { ibv_free_device_list(list) };
}
```

执行，预期会打印：`device name: rxe_0`。完整代码参考 https://github.com/SF-Zhou/rdma-demo。
