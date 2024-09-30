---
title: Locks
tags:
  - Rust
  - Concurrency
  - Locks
  - wtfacademy
---

# WTF Rust 极简入门: 使用锁管理并发

在并发编程中，锁是用于管理多个线程间对共享资源访问的同步机制。在 Rust
中，互斥锁（Mutex）和读写锁（RwLock）是最常用来控制线程对数据安全访问的两种锁。本章将详细介绍这两种锁的使用方法和最佳实践。

## 互斥锁（Mutex）

`Mutex`，全称为互斥锁（Mutual Exclusion），其核心功能是保证同一时间内只有一个线程可以访问某个数据，防止因并发访问而产生数据竞争的问题。

### 基本使用

当你使用 `Mutex` 时，需要调用 `lock` 方法来访问数据独占权。这个方法会阻塞其他线程访问共享数据直到获取到锁。由于 `Drop`
的调用，一旦线程完成数据操作，锁会在作用域结束时自动释放。

#### 示例

```rust
use std::sync::{Mutex, Arc};
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

### 注意事项

- **死锁**：尝试在一个线程中多次锁定同一个 `Mutex` 或者在多个线程间以不同顺序锁定多个 `Mutex`，可能会导致死锁。
- **避免长时间持锁**：长时间持锁会降低应用性能，确保锁的作用域尽可能小。
- **错误处理**：`lock` 方法返回的是一个 `Result` 类型，它可能因为锁被中毒（poisoned，即某个持有锁的线程在释放锁前恐慌退出了）而出错。

## 读写锁（RwLock）

`RwLock` 允许多个线程读取数据，但写入数据时需要独占访问。适用于读多写少的场景，可以大幅提升应用性能。

### 基本使用

读取数据时使用 `read()` 方法获取锁，写入数据时使用 `write()` 方法。这两种方法都会返回一个被锁保护的数据的可变或不可变引用。

#### 示例

```rust
use std::sync::{RwLock, Arc};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    // 创建读线程
    let reader = Arc::clone(&data);
    let reader_thread = thread::spawn(move || {
        let read_data = reader.read().unwrap();
        println!("Reader: {}", read_data);
    });

    // 创建写线程
    let writer = Arc::clone(&data);
    let writer_thread = thread::spawn(move || {
        let mut write_data = writer.write().unwrap();
        *write_data += 1;
    });

    reader_thread.join().unwrap();
    writer_thread.join().unwrap();

    // 最终数据
    println!("Final data: {}", *data.read().unwrap());
}
```

### 注意事项

- **写锁饥饿（Writer Starvation）**：发生在写者尝试获取锁时因为持续的读操作而被不断延迟的情况。在许多RwLock实现中，为了保证读操作的高效性，允许多个读者同时持有锁，这可能导致写者长时间等待。
- **读锁饥饿（Reader Starvation）**：当 `RwLock`
  实现倾向于写锁时，可能会出现读锁饥饿的情况。这意味着一旦有写者在等待，即使锁当前可用，新的读请求也会被阻塞，直到写锁被释放。这种策略可以防止写锁饥饿，但如果写请求频繁，读者可能会遭受长时间的等待。
- **锁升级和降级**：Rust 的标准库 `RwLock` 不支持锁升级（持有读锁的情况下请求写锁）或锁降级（持有写锁的情况下转换为读锁）。

通过使用这些锁，你可以在 Rust 中安全地管理并发访问共享数据。正确地使用锁可以帮助你避免多线程编程中常见的问题，如数据竞争、死锁和并发性能问题。