# 所有权、借用、引用
---
title: 4. Ownership
tags:
- Rust
- basic
- wtfacademy
---

## 泛型
泛型是允许编写可用于多种数据类型的函数和数据结构的代码。与C或C++中的模板类似，泛型提高了代码的复用性，同时减少了冗余。

### 泛型示例：函数

考虑一个找到切片中最大元素的函数。没有泛型，你可能需要为每种数据类型编写一个不同的函数。使用泛型，你可以写一个函数适用于任意数据类型：

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);
println!("The largest number is {}", result);

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);
println!("The largest char is {}", result);
```

在这个例子中，`largest`函数通过`<T: PartialOrd>`约束来接收任意实现了`PartialOrd` trait的类型`T`的切片——这意味着`T`是可部分排序的。`PartialOrd`是Rust标准库提供的一个trait，表示类型值是可比较的。

#### 泛型结构体示例

泛型同样可以应用于结构体的定义中，让我们定义一个点`Point`来存储两种可能不同类型的`x`和`y`坐标：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

let integer_and_float = Point { x: 5, y: 4.0 };
println!("Point coordinates: ({}, {})", integer_and_float.x, integer_and_float.y);
```

这个`Point<T, U>`结构体定义展现了如何使用泛型来存储两种类型的数据。它能够适用于既不严格规定`x`和`y`必须相同类型的场景。

## Trait（特性）

Trait可以被认为是定义一组方法的方式，这些方法可以被多种类型实现。这有点像其他语言中的接口。

### Trait示例

这里我们定义一个`Summary` trait，它要求实现它的类型必须提供一个返回摘要字符串的方法：

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.title, &self.content[..10])
    }
}

let article = Article {
    title: "Rust Language".to_string(),
    content: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
};

println!("New article available! {}", article.summarize());
```

## 生命周期

生命周期是Rust的一个有点复杂但非常强大的特性，用于确保引用在被使用时始终有效。

### 生命周期示例：函数

让我们看一个带有生命周期的函数例子，该函数接受两个字符串切片引用并返回其中一个：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let string1 = String::from("abcd");
let string2 = "xyz";

let result = longest(string1.as_str(), string2);
println!("The longest string is {}", result);
```

生命周期注解`'a`指出了所有输入引用和返回引用都必须拥有相同的生命周期——这确保了返回的引用在其使用期间保持有效。

希望这些更详细的解释和例子有助于你更好地理解泛型、trait（特性）和生命周期。这些概念是Rust高效安全性的基石，理解它们对于成为一个高效的Rust程序员至关重要。