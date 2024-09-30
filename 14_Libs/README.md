---
title: 常用 Rust 库介绍
tags:
- Rust
- libraries
- wtfacademy
---

# WTF Rust 极简入门：常用 Rust 库介绍

Rust生态系统中有许多强大且实用的库，这些库可以帮助开发者提高开发效率，增加项目功能。在本章节中，我们将探讨一些在 Rust 开发中常用的库，并简要介绍它们的功能和如何在项目中使用它们。

## 1. serde

[serde](https://github.com/serde-rs/serde) 是一个序列化和反序列化框架，非常适合处理JSON、YAML、Bincode等多种数据格式。它通过提供`Serialize`和`Deserialize`trait来自动处理数据转换，极大简化了数据处理过程。

### 如何使用serde

首先，将`serde`及其相关的依赖添加到你的`Cargo.toml`中：

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

在 Rust 文件中，你可以这样使用它来序列化和反序列化数据：

```rust
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_member: bool,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_member: true
    };

    // 序列化
    let serialized = serde_json::to_string(&person).unwrap();
    println!("serialized = {}", serialized);

    // 反序列化
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

## 2. reqwest

[reqwest](https://github.com/seanmonstar/reqwest) 是一个 Rust 的 HTTP 客户端库，用于执行 HTTP 请求。它提供了异步和阻塞两种调用方式，支持多种 HTTP 方法和自定义头部，非常适合需要网络功能的应用。

### 如何使用reqwest

在`Cargo.toml`中添加：

```toml
[dependencies]
reqwest = "0.11"
tokio = { version = "1", features = ["full"] }
```

使用`reqwest`发送 GET 请求：

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.github.com/users/octocat")
        .await?
        .text()
        .await?;

    println!("Response: {}", resp);
    Ok(())
}
```

## 3. tokio

[tokio](https://github.com/tokio-rs/tokio) 是一个异步运行时，它提供了编写高性能网络应用程序所需的所有工具。它是许多异步应用和库的基础。

### 如何使用tokio

在`Cargo.toml`中添加：

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

使用`tokio`运行异步代码：

```rust
#[tokio::main]
async fn main() {
    let task_one = tokio::spawn(async {
        println!("Task one executed");
    });

    let task_two = tokio::spawn(async {
        println!("Task two executed");
    });

    tokio::join!(task_one, task_two);
}
```

## 4. axum

[axum](https://github.com/tokio-rs/axum) 是一个基于 Tokio 和 Tower 服务栈的异步 Web 框架。它在 Rust 生态系统中具有特殊地位，并提供了路由、中间件、状态共享等核心功能，旨在简化 Web 应用的开发过程。

### 如何使用axum

在`Cargo.toml`中添加：

```toml
[dependencies]
axum = "0.7.5"
tokio = { version = "1", features = ["full"] }
```

以下为一个 Axum 的 hello world 案例：

```rust
use axum::{routing::get, Router};

async fn hello_handler() -> &'static str {
    "Hello, Axum!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 5. clap

[clap](https://github.com/clap-rs/clap) 是Rust的命令行参数解析器，主要用于构建功能完备的命令行工具。

### 如何使用clap

在`Cargo.toml`中添加：

```toml
[dependencies]
clap = { version = "4.5.4", features = ["derive"] }
```

以下为一个简单案例：

```rust
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.name);
}
```

## 总结

本章介绍了一些 Rust 中常见且功能强大的库，这些库在实际项目中可以极大地帮助开发者提高编码效率和项目质量。Rust的生态系统持续增长，了解并能够熟练使用这些库将为你的 Rust 编程之路加分。