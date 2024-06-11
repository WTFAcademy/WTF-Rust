---
title: Channel 通道
tags:
- Rust
- concurrency
- wtfacademy
---

# WTF Rust 极简入门: 消息传递并发性
多线程间共享、传递数据有多种方式，最常用的方式就是通过消息传递或者将锁和 `Arc` 联合使用。对于前者，典型的如Erlang语言，以及Go语言。Rust 鼓励通过消息传递来实现线程间通信，而不是通过共享内存。这种方式遵循了“Do not communicate by sharing memory; instead, share memory by communicating”的并发哲学。通过 std::sync::mpsc 提供的通道（Channel），多个线程可以安全地传递数据，避免了复杂的锁机制。

## 使用通道

通道是 Rust 中用于线程间通信的主要机制，由发送者（sender）和接收者（receiver）组成。通道确保了数据传输的安全性，防止了数据竞争和其他并发错误。

### 创建通道

使用`std::sync::mpsc`模块可以创建一个通道，mpsc是多个生产者，单个消费者(multiple producer, single consumer)的缩写：

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello from the thread!");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

### 注意事项

1. **发送端的克隆**：如果多个线程需要发送消息到同一个接收者，可以克隆发送端 `tx.clone()`。每个发送端都可以在不同的线程中使用，但所有消息仍将发送到同一个接收端。
2. **错误处理**：`send`方法返回 `Result<T, E>`，如果接收端已关闭，则返回 `Err`，需要适当处理这种情况。
3. **阻塞与非阻塞接收**：`recv`方法阻塞当前线程直到收到消息，而 `try_recv` 不会阻塞，它立即返回 `Result` ，根据是否有消息可用来决定是 `Ok` 还是 `Err` 。

### 日常使用场景

- **日志系统**：可以将日志信息从多个线程发送到一个日志处理线程，集中处理。
- **任务分发系统**：主线程创建任务并通过通道分发给工作线程，工作线程处理完任务后，再将结果发送回主线程。
- **事件通知系统**：在事件触发时，通过通道通知相关的处理线程进行相应的操作。

## 高级通道操作

以上基本用法之外，Rust 的通道系统还支持更复杂的操作：

### 多消费者模式

虽然`std::sync::mpsc`是多生产者单消费者模型，但你可以通过创建多个接收端和手动分配的方式管理多消费者的场景。

### 选择器（Selectors）

通过选择器（selectors），可以在多个接收端之间进行选择，处理第一个可用的消息。这对于优先级消息处理尤其有用，使用`select!`宏可以实现：

```rust
use std::sync::mpsc::{self, Select};
use std::thread;

fn main() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let sel = Select::new();
    let mut handle1 = sel.handle(&rx1);
    let mut handle2 = sel.handle(&rx2);

    unsafe {
        handle1.add();
        handle2.add();
    }

    thread::spawn(move || {
        tx1.send(1).unwrap();
    });

    thread::spawn(move || {
        tx2.send(2).unwrap();
    });

    let oper = sel.select();
    let index = oper.index();
    let result = oper.recv().unwrap();

    println!("Received {} from channel {}", result, index);
}
```

通过这些高级功能，可以灵活地在 Rust 程序中处理并发问题，为高效、可靠的系统设计提供强大支持。