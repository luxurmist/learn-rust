<!--
    file: get-started.md
    author: luxurmist
    date: 2026-04-02
-->

# Rust快速入门

## 1. Rust开发环境

安装`Rust`的主要方式是通过`rustup`，它是`Rust`的安装器与版本管理工具。

### 1.1 rustup安装

在`Linux`上：
```console
curl https://sh.rustup.rs -sSf | sh
```

在 `Windows` 上，[官方安装教程](https://rust-lang.org/tools/install/)。

默认安装路径为 `~/.cargo` 目录，可通过设置 `RUSTUP_HOME` 和 `CARGO_HOME` 环境变量来更改默认路径。

默认安装 `rustup` 本身和 `rustc` （编译器）和 `cargo` （包管理工具）的标准版本。

### 1.2 rustup基本命令

#### 安装其他版本
```console
rustup install x.xx.xx //标准版
rustup install nightly-xxxx-xx-xx //夜版

//三种版本的最新版
rustup install stable
rustup install beta
rustup install nightly
```

#### 升级所有安装版本
```console
rustup update
```

#### 设置默认版本
```console
rustup toolchain default nightly //设置非标准版为默认版本
```

## 换源
[USTC](https://mirrors.ustc.edu.cn/help/rust-static.html)