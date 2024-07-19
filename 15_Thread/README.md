---
title: 使用线程
tags:
- Rust
- concurrency
- wtfacademy
---

# WTF Rust 极简入门: 使用线程

并发和多线程编程是现代软件开发的重要组成部分，特别是在需要高效处理大量数据或执行多任务操作的应用中。Rust因其内存安全的保证而成为处理并发任务的理想选择。在本章中，我们将探讨 Rust 中多线程的使用方法和一些基本概念。

## 使用线程
Rust 的 `std::thread` 模块是标准库中用于并发编程的核心工具，提供了创建和管理操作系统线程的功能。通过 `std::thread::spawn`，用户可以方便地启动新线程，并在新线程中运行闭包代码。该模块利用 Rust 独特的所有权和借用系统，确保线程间的数据共享是安全的，从而有效避免数据竞争和其他并发错误。用户可以通过线程句柄（Handle）来管理线程的生命周期，使用 `join` 方法阻塞主线程，直到子线程执行完毕。此外，配合 `std::sync` 模块中的 `Arc` 和 `Mutex` 等工具，`std::thread` 可以实现复杂的线程同步与通信，为开发高效、安全的并发程序提供了坚实的基础。这些特性使得 Rust 的并发编程既强大又安全，适用于各种高性能应用场景。

### 创建线程

要在 Rust 中创建新线程，可以使用`std::thread::spawn`函数。这个函数接受一个闭包，闭包内包含将在线程上执行的代码。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
在上面的代码中，我们创建了一个打印某些内容的新线程，同时在主线程打印其他内容。但是需要注意的是，这种编写方式，当主线程结束的时候，新线程也会执行结束，不管其是否执行完毕，我们可以使用join来等待所有线程结束。

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // 在新线程中执行的代码
        for i in 1..10 {
            println!("线程中打印：{}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    // 在主线程中执行的其他操作
    for i in 1..5 {
        println!("主线程中打印：{}", i);
        thread::sleep(std::time::Duration::from_millis(2));
    }

    // 等待新线程结束
    handle.join().unwrap();
}
```

在这个示例中，我们创建了一个线程，它在一个循环中打印一些信息。同时，主线程也在进行类似的操作。`thread::spawn`的返回值类型是`JoinHandle`，`JoinHandle`是一个拥有所有权的值，对其使用`join()`方法来保证子线程完成后主线程才会继续执行，这是避免子线程在程序退出时未完全执行完成的常用方式。

### 线程和所有权

Rust的所有权规则同样适用于线程。如果你需要在创建的线程中使用某些数据，你需要确保数据的所有权已经转移到线程中。对于可变数据，通常需要使用多线程安全的类型，如`Arc<Mutex<T>>`。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

在这个示例中，我们使用`Arc`来共享数据，`Mutex`来实现线程间的同步，确保每次只有一个线程能修改数据。这样可以安全地在多个线程间共享和修改可变数据。

## 总结

本节简要介绍了如何在 Rust 中使用线程来实现并发执行。通过理解和利用 Rust 的所有权和类型系统，你可以构建安全且高效的并发应用程序。在接下来的小节中，我们会继续探讨 Rust 中的其他并发模型和技术。