# 华南理工大学软件学院《数字逻辑》课程软件安装和使用说明

### 1. 软件介绍

[Vivado](https://en.wikipedia.org/wiki/Xilinx_Vivado) 是 [Xilinx](https://www.xilinx.com/) 公司出品的一款[硬件设计语言](https://en.wikipedia.org/wiki/Hardware_description_language)综合（synthesis）及分析软件。在《数字逻辑》课程中，将使用 Vivado 软件，配合 [Basys3 开发板](https://reference.digilentinc.com/_media/basys3:basys3_rm.pdf)进行基本数字逻辑电路的构建和测试。

### 2. 软件安装

Vivado 软件拥有众多版本，但由于实验室计算机操作系统 Windows XP 版本限制，能使用的 Vivado 版本上限为 2014.2。这也是推荐大家在自己电脑上安装的版本（有需要的话）。

由于安装包过大，这里提供校内的下载地址（校内教育网可用）。由于 IP 为动态 IP，进而可能发生下载链接的改变，如有下载需求请持续关注本页面。

1. 下载 Vivado 软件：[Vivado 2014.2](http://116.56.129.146/Xilinx_Vivado_SDK_Win_2014.2_0612_1.tar.gz)
2. 下载 Vivado License：[Xilinx.lic](http://116.56.129.146/Xilinx.lic)

如果上述地址不可用，也可以通过科学上网的方式，前往官网注册、下载。地址：https://www.xilinx.com/support/download/index.html/content/xilinx/en/downloadNav/vivado-design-tools/2014-2.html。

下载完安装包后，解压缩，找到 xsteup.exe，双击运行安装。在选择安装的版本时，选择 Vivado Webpack。这是 Xilinx 提供的免费版本，可以通过上面的正版 License 激活使用。其他选项均默认、同意即可。

安装完成后将自动打开 Vivado License Manager。如果没有打开或意外关闭可自行在开始菜单中找到并打开。找到 Manage License，点击第一个框内的 Copy License，最后选择下载好的 Xilinx.lic 即可完成激活。
