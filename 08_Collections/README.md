---
title: 8. Collection
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 集合

推特：[@0xAA_Science](https://twitter.com/0xAA_Science)

社区：[Discord](https://discord.gg/5akcruXrsk)｜[微信群](https://docs.google.com/forms/d/e/1FAIpQLSe4KGT8Sh6sJ7hedQRuIYirOoZK_85miz3dw7vA1-YjodgJ-A/viewform?usp=sf_link)｜[官网 wtf.academy](https://wtf.academy)

所有代码和教程开源在 github: [WTF-Rust](https://github.com/WTFAcademy/WTF-Rust)

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

在Rust中,有两种常见的字符串类型: `String`和`&str`。`String`是一个可增长的、可变的、有所有权的`UTF-8`编码字符串类型,而`&str`是一个指向有效`UTF-8`序列的切片(`slice`)。`&str`通常用于函数参数中,而`String`则用于需要所有权或可变性的场景,如需要修改字符串的情况。

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
### 字符串与切片

```rust
fn main() {
    let s1: &str = "Hello, world!"; // 字符串字面量的类型是 &str
    let s2: &str = &s1[0..5]; // 使用切片语法创建一个子字符串
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3: String = s1.to_string(); // 将 &str 转换为 String
    println!("s3: {}", s3);

    print_str(s1); // 将 &str 传递给函数
}

fn print_str(s: &str) {
    println!("Inside print_str: {}", s);
}
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

哈希 Map 在线性表搜索情况下提供了更快的数据访问速度，尤其是对大数据集非常高效。

## 小结

本章为你介绍了 Rust 在处理不同集合类型上的能力。理解并掌握这些集合的操作将大大增强你处理数据集时的灵活性和效率。在应对日常编程问题时，合理使用集合类型对于写出高效、易维护的代码至关重要。希望这一章的内容能帮助你在 Rust 旅程中迈出坚实的一步。