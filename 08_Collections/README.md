---
title: 8. Collection
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 8 集合

本章专注于 Rust 中的集合类型，这些类型是在开发中经常会用到的数据结构。我们将探索 `Vector`、字符串和 `HashMap`，了解它们的使用方法和各自的特点。这些集合类型为存储和访问数据集提供了多种灵活的方式。

## 1. 使用Vector

`Vector`（通常简称为 `vec`）是一个可增长的数组，可以存储同类型的值。与数组不同，向量的大小可以在运行时改变。

### 创建和使用向量

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();  // 创建一个空的向量
    v.push(5);                        // 向向量中添加元素
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);       // 打印向量
}
```

你也可以使用宏 `vec![]` 快速初始化一个向量：

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];       // 使用宏创建并初始化向量
    println!("Vector: {:?}", v);
}
```

遍历向量：

```rust
for i in &v {
    println!("{}", i);
}
```

## 2. 字符串处理

在 Rust 中，`String` 类型被用来存储 UTF-8 编码的文本。字符串的可变性和所有权特性使其操作既灵活又符合 Rust 的安全性要求。

### 创建和修改字符串

```rust
fn main() {
    let mut s = String::new();        // 创建一个空字符串
    s.push_str("Hello");              // 添加文本
    s.push(' ');                      // 添加单个字符
    s.push_str("world!");

    println!("String: {}", s);
}
```

连接字符串：

```rust
let s1 = "Hello".to_string();
let s2 = " World!";
let s3 = s1 + s2;  // 注意：s1 在这里被移动了，不能再被使用

println!("Combined string: {}", s3);
```

## 3. 哈希Map

`HashMap` 用于存储键值对集合，其中键和值可以是不同类型。这是一种非常有用的数据结构，可以快速访问和管理关联数据。

### 创建和使用 HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    if let Some(score) = scores.get("Blue") {
        println!("Blue team's score: {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

哈希Map在线性表搜索情况下提供了更快的数据访问速度，尤其是对大数据集非常高效。

## 小结

第五章为你介绍了 Rust 在处理不同集合类型上的能力。理解并掌握这些集合的操作将大大增强你处理数据集时的灵活性和效率。在应对日常编程问题时，合理使用集合类型对于写出高效、易维护的代码至关重要。希望这一章的内容能帮助你在 Rust 旅程中迈出坚实的一步。如果有任何问题，欢迎随时提出！