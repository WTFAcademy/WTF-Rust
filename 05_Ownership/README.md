---
title: 5. Ownership
tags:
  - Rust
  - basic
  - wtfacademy
---

# WTF Rust极简入门: 所有权、借用、引用
本章是`Rust`语言精髓的核心所在：所有权、借用和引用。这些概念是`Rust`安全内存管理的基石，帮助开发者写出安全且高效的代码，避免了传统语言中常见的错误，如空指针访问和数据竞争。

## 1. 所有权规则

在`Rust`中，所有权系统的核心规则可以归纳为三条：

- 每一个值都被其所有者变量所拥有。
- 值在任意时刻只能被一个所有者拥有。
- 当所有者离开作用域时，值将被丢弃。

这些规则确保内存安全无泄漏，同时避免手动管理内存。

### 示例：所有权转移

```rust
fn main() {
    let s1 = String::from("hello");  // s1 是 hello 对象的所有者

    let s2 = s1;                     // 所有权从 s1 转移至 s2,s1将变得不可用
    // println!("{s1}");             // 错误：s1 不再持有字符串

    display(s2);                    //s2 将所有权转让给函数参数s，s2将变得不可用
    // println!("{s2}");             // 错误：s2不可用
}

fn display(s:String){
   println!("{:?}",s);
}
```

## 2. 借用

在`Rust`中，借用是指通过引用来获得数据的**访问权**，而不是**所有权**，用符号 `&` 表示。借用使得可以在不转移所有权的情况下，让多个部分同时访问相同的数据。借用分为两种：不可变借用和可变借用。

- **不可变借用**：允许多次借用，但借用期间不能修改数据，`Rust` 中默认为不可变借用。
- **可变借用**：允许数据被修改，但在同一时间内只能有一个可变借用。

### 示例：不可变借用

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 发生不可变借用，函数只能读取但不能修改 s1

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s 是对 s1 的引用
    s.len()
}
```

### 示例：可变借用

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);  // s 被可变借用，函数可以修改 s

    println!("{}", s);
}

fn change(some_string: &mut String) -> &mut String{
    some_string.push_str(", wtf!");
    some_string
}
```

## 小结

通过如上例子你应该对`Rust`的核心概念：所有权、借用有了基本的理解。掌握这些概念对于高效利用`Rust`语言特性至关重要，可以帮你编写出更安全和高效的代码。如果有任何疑问或需要进一步的解释，请随时提出！