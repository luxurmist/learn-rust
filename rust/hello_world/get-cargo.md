<!--
    file: hello_cargo.md
    author: luxurmist
    date: 2026-04-02
-->

# Cargo使用

## 1. 开始使用

### 1.1 创建新项目

```console
// new命令会创建一个新的项目目录
$ cargo new hello_world --bin //默认
$ cargo new hello_world --lib
$ cargo new hello_world --vcsnone //默认会创建git存储库，如不需要传递此参数
```

### 1.2 编译与运行

```console
// 下面命令在项目目录下使用
$ cargo build //编译
$ cargo run //编译并运行
$ cargo build --release //优化编译
```

### 1.3 依赖项

在 `Cargo.toml` 文件中添加 `[dependencies]` 部分，在下面列举所需依赖。
```toml
[dependencies]
rand = "0.10.0"
bevy = "0.18.1"
```
添加依赖后运行 `cargo build` 会获取新的项目依赖与依赖的依赖进行编译，并更新 `Cargo.lock` 文件，这个文件包含依赖项的版本信息，不需要也不应手动编辑。

版本不会自动更新，需要用 `cargo update` 命令手动更新依赖版本。

### 1.4 项目结构

- `Cargo.toml` 和 `Cargo.lock` 存储在项目根目录。
- 源码存储在 `src` 目录，默认库文件 `src/lib.rs` ，默认可执行入口 `src/main` ,其他可执行文件 `src/bin` 。
- 测试在 `tests` 目录。
- 示例在 `examples` 目录。

## 2. 基础概念

### 2.1 包与Crate

`crate` 是Rust编译器每次处理的最小代码单位。
crate有两种形式：二进制crate 和 库crate。
`crate root` 是一个源文件，Rust编译器以它为起始点，并构成crate 的根模块，是 `src/main.rs` 或 `src/lib.rs` 。
包 package 是提供一系列功能的一个或者多个 `crate` 的捆绑，一个包会包含一个 `Cargo.toml` 文件。

### 2.2 模块

#### 模块声明
在 crate 根文件中使用 `mod` 声明一个新模块。
编译器在寻找模块代码路径：
- 内联， `mod mod_name{}`
- src/mod_name.rs
- src/mod_name/mod.rs

#### 子模块
在除根节点以外的任何文件中，可以定义子模块。一个模块的代码默认对其父模块私有。使用 `pub mod` 公开模块，模块内需要公开的成员同样是加 `pub` 声明。
编译器在寻找子模块代码路径：
- 内联， `mod sub_mod_name{}`
- src/mod_name/sub_mod_name.rs
- src/mod_name/sub_mod_name/mod.rs

#### 访问
一个模块如果是crate的一部分，在隐私规则允许下，可以在这个crate内的任何地方引用该模块的代码。
```rust
crate::mod_name::sub_mod_name::any_type
```

可以使用 `use` 关键字减少长路径的重复。 
