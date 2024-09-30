---
title: Asynchronous Rust – Basics & Futures and Tasks
tags:
- Rust
- Asynchronous Programming
- wtfacademy
---

# 异步 Rust

在本章中，我们将深入探讨 Rust 中的异步编程基础和 Future 及任务的概念和应用。理解和掌握这些概念是构建高效异步程序的关键。

## 异步编程基础

异步编程是处理并发编程的一种高效方式，特别适合 I/O 密集型或需要高响应速度的应用。在 Rust 中，异步编程主要依赖于 `async`
关键字标记的异步函数和 `await` 表达式。

### 异步函数与 `async/await`

- **异步函数（`async fn`）**：使用 `async` 关键字定义。这种函数调用不是直接返回结果，而是返回一个实现了 `Future` trait 的对象。
- **等待（`await`）**：用于在异步函数内部暂停执行当前任务，并等待其他 `Future` 完成。当 `Future` 完成时，程序将从暂停的点继续执行。

#### 示例：基础异步函数

```rust
async fn perform_task() -> String {
    "hello async".to_string()
}

#[tokio::main]
async fn main() {
    let result = perform_task().await;
    println!("{}", result);
}
```

这个简单的示例演示了如何定义和运行异步函数。`#[tokio::main]`是一个宏，为主函数提供异步运行时环境。

## Future 和任务

在 Rust 的异步编程中，`Future` 是一个核心概念，它代表了一个尚未完成的计算。

### `Future` 概念

`Future` trait 定义了一个异步操作，这个操作可能还没有完成。`Future` 可以通过调用 `poll` 函数来推进，该函数决定是否继续等待或者输出最终结果。如果
`Future` 完成了，将返回 `Poll::Ready(result)`。如果 `Future` 还不能完成，将返回 `Poll::Pending` 并安排在 `Future`
准备好做出更多进展时调用 `wake()` 函数。当 `wake()` 被调用时，驱动 `Future` 的执行器将再次调用 `poll`，以便 `Future`
可以取得更多进展。

#### 示例：简化版本的Future

```rust
trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
```

#### 示例：普通版本的Future

```rust
trait Future {
    type Output;
    fn poll(
        // 注意从 &mut self 到 Pin<&mut Self> 的变化：
        self: Pin<&mut Self>,
        // 以及从 wake: fn() 到 _cx: &mut Context<'_> 的变化：
        _cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
```

### 异步任务的运行

异步任务是由 `Future` 变体驱动的，通常是通过一个执行器来调度和执行。执行器负责管理 `Future` 的状态并在合适的时候调用
`poll` 方法。

#### 示例：手动实现自定义Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SimpleFuture;
impl Future for SimpleFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready("completed".to_string())
    }
}

#[tokio::main]
async fn main() {
    let my_future = SimpleFuture;
    let result = my_future.await;
    println!("Future result: {}", result);
}
```

在这个例子中，`SimpleFuture` 是一个自定义的 `Future`，它立即返回一个结果，没有延时或等待。通过使用 `await`，我们可以在异步函数中等待这个
`Future` 完成。

通过对这些基础和核心概念的理解，你可以开始构建更复杂的异步应用程序，这些程序能够有效地执行多个操作，而不会互相阻塞。在未来的节中，我们将探讨如何结合多个
`Future` 并发处理复杂的异步逻辑。