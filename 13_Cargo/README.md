---
title: Cargo
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: Cargo 管理

在本章中，我们将讨论如何使用Rust的包管理工具Cargo来管理项目依赖。我们将首先探讨Cargo的基本功能，然后深入研究如何添加、更新和管理外部库依赖项。最后，我们会介绍一些Cargo的高级功能，如如何使用工作空间管理多个包。

## 理解Cargo

Cargo是Rust的官方包管理工具，负责多方面的任务：编译代码、下载依赖库、构建软件包等。它还能够确保所有依赖项的版本兼容性，让项目构建更可靠。

###  初始化项目

要开始使用Cargo管理Rust项目，首先需要创建一个Cargo配置文件（`Cargo.toml`）。这个文件描述了项目的基本信息和依赖。使用命令`cargo new <project_name>`能够快速生成一个新的项目目录，里面包含了基本的项目结构和`Cargo.toml`文件。

## 添加和管理依赖

在`Cargo.toml`文件中，你可以在`[dependencies]`节下添加所需的库依赖。Cargo支持从crates.io（Rust的官方包仓库）、GitHub或本地路径等多种来源获取依赖。

### 示例：添加一个依赖

假设你想要添加一个名为"serde"的序列化库，你可以在`Cargo.toml`的`[dependencies]`节下添加以下代码：

```toml
[dependencies]
serde = "1.0"
```

然后运行`cargo build`，Cargo会自动下载并编译serde及其依赖。

### 更新依赖

更新依赖只需修改`Cargo.toml`中的版本号，然后运行`cargo update`，Cargo将解析并更新项目依赖。

## 使用Cargo的工作空间

对于包含多个包的大型项目，使用Cargo的工作空间功能能够高效管理依赖与构建。工作空间由一个顶层的`Cargo.toml`配置文件定义，它链接到多个子项目。

### 创建工作空间

在工作空间的根目录的`Cargo.toml`中，你可以如下配置子包：

```toml
[workspace]
members = [
    "crate1",
    "crate2",
]
```

每个成员目录都有自己的`Cargo.toml`文件和源代码。这样设置后，Car工作空间中的所有包能共享依赖，提高编译效率。

## 高级功能

### 特性标志

Cargo支持通过特性标志来启用或禁用代码中的特定功能。这在需要可选功能的情况下非常有用。

### Cargo插件

Cargo支持使用第三方插件来扩展其功能，例如`cargo-edit`可以直接在命令行中添加、更新和删除依赖。

通过这一章的学习，你应该对如何使用Cargo来管理Rust项目的依赖有了全面的了解。掌握这些知识将帮助你更高效地开发Rust应用。