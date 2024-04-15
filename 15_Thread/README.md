---
title: 使用线程
tags:
- Rust
- concurrency
- wtfacademy
---

# WTF Rust 极简入门: 使用线程

并发和多线程编程是现代软件开发的重要组成部分，特别是在需要高效处理大量数据或执行多任务操作的应用中。Rust因其内存安全的保证而成为处理并发任务的理想选择。在本章中，我们将探讨Rust中多线程的使用方法和一些基本概念。

## 使用线程

在Rust中，线程可以通过标准库中的`std::thread`模块创建和管理。`std::thread`提供了一种创建执行线程的简单方法。

### 创建线程

要在Rust中创建新线程，可以使用`std::thread::spawn`函数。这个函数接受一个闭包，闭包内包含将在线程上执行的代码。

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

在这个示例中，我们创建了一个线程，它在一个循环中打印一些信息。同时，主线程也在进行类似的操作。使用`join()`方法来保证子线程完成后主线程才会继续执行，这是避免子线程在程序退出时未完全执行完成的常用方式。

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

本节简要介绍了如何在Rust中使用线程来实现并发执行。通过理解和利用Rust的所有权和类型系统，你可以构建安全且高效的并发应用程序。在接下来的小节中，我们会继续探讨Rust中的其他并发模型和技术。