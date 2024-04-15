---
title: Errors
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 错误处理

在任何编程任务中，处理潜在的错误和异常是至关重要的。Rust 通过一些独到的方式，提供了强大而灵活的错误处理机制，旨在编写更为安全和可预测的代码。此章将深入探索 Rust 中的错误处理方法，包括 `panic!` 宏、`Result` 类型以及如何传播错误。

## 1. `panic!` 宏

在 Rust 中，当程序遇到无法处理的错误时，可以调用 `panic!` 宏。这会导致程序清理其在当前线程的堆栈，并立即终止程序。

### 示例：触发 Panic

```rust
fn main() {
    panic!("This is a critical error!");
}
```

这段代码将在执行时触发 panic，打印出错误信息并终止程序。在开发初期，`panic!` 是一种快速的错误反馈方式，但在最终的产品代码中，通常使用更为精细的错误处理机制。

## 2. `Result` 类型

Rust 通过 `Result` 类型提供了一种标准化的方式来处理可能成功或失败的操作。`Result` 类型是一个枚举，具有两个变体：`Ok` 和 `Err`，分别表示操作的成功或失败。

### 示例：使用 `Result`

```rust
fn divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
```

在这个示例中，`divide` 函数尝试进行除法运算，如果除数为零，则返回 `Err`。

## 3. 错误传播

当你希望从函数中返回错误以便调用者可以决定如何处理时，Rust 允许你使用 `?` 运算符轻松传播错误。

### 示例：错误传播

```rust
fn get_data_from_file() -> Result<String, std::io::Error> {
    let path = "data.txt";
    let content = std::fs::read_to_string(path)?;

    Ok(content)
}

fn main() {
    match get_data_from_file() {
        Ok(data) => println!("File content: {}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }
}
```

如果 `read_to_string` 调用失败，`?` 运算符会立即从 `get_data_from_file` 函数中返回 `Err`，包含相应的错误信息。

## 4. 自定义错误

为了更细致地控制错误处理，你可以使用自定义类型表示错误。

### 示例：定义自定义错误

```rust
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

fn risky_operation() -> Result<(), MyError> {
    if true { // 假定这里是一些业务逻辑的结果
        Err(MyError { message: String::from("Something went wrong") })
    } else {
        Ok(())
    }
}
```

在这个示例中，`MyError` 结构体实现了 `fmt::Display`，可以用来输出错误信息。

## 小结

本章介绍了 Rust 中的错误处理机制，包括如何使用 `panic!` 进行紧急处理，如何利用 `Result` 处理可以恢复的错误，以及如何传播和定义自定义错误。理解和正确应用这些错误处理技术对于编写健壮和可维护的 Rust 程序至关重要。