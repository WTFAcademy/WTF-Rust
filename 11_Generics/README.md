---
title: Generics
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 泛型

泛型是 Rust 程序设计中的一项核心功能，它允许程序员编写灵活且可重用的代码。通过使用泛型，你可以编写函数和数据结构，它们可以适用于多种类型，而不需针对每种类型编写重复的代码。这一节将深入探讨泛型，并展示几个实际的应用场景来阐释其强大之处。

## 泛型基础

泛型的基本用法包括定义泛型函数、结构体、枚举和方法。使用泛型时，你定义的函数和结构体可以处理多种数据类型。

### 泛型函数

泛型函数可以对不同的数据类型执行相同的逻辑。这避免了为每个数据类型编写单独的函数。

```rust
fn display<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

fn main() {
    display(42); // 显示整数
    display("Hello, Rust!"); // 显示字符串
}
```

### 泛型结构体

同样，泛型也可以用在结构体定义中，以便结构体可以用各种数据类型初始化。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}
```

### 泛型枚举

泛型枚举允许你定义适用于多种类型的枚举。例如：

```rust
// 定义一个泛型枚举 Option，用于表示一个可能包含值的类型
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = MyOption::Some(42);
    let no_number: MyOption<i32> = MyOption::None;

    match some_number {
        MyOption::Some(value) => println!("We have a number: {}", value),
        MyOption::None => println!("No number found"),
    }
}

```

在这个例子中，`MyOption` 枚举是泛型的，接受一个类型参数 `T`，可以表示一个可能包含值的类型。

### 泛型方法

泛型方法允许你在方法中使用泛型类型。例如：

```rust

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let point = Point::new(5, 10);
    println!("Point x: {}", point.x());
}


```

在这个例子中，`Point` 结构体和它的 `new` 方法、`x` 方法都是泛型的，接受一个类型参数 `T`。

## 泛型实例场景

### 泛型在集合中的应用

使用泛型可以创建灵活的数据结构，比如栈（Stack）。

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn main() {
    let mut stack_of_int = Stack::new();
    stack_of_int.push(1);
    stack_of_int.push(2);

    let mut stack_of_str = Stack::new();
    stack_of_str.push("Hello");
    stack_of_str.push("World");

    println!("{:?}", stack_of_int.pop());
    println!("{:?}", stack_of_str.pop());
}
```
这个例子展示了如何使用泛型来创建一个可以存储任何类型的栈。

### 泛型与 Trait 约束

泛型常常与 trait 约束一起使用，以确保泛型类型具备某些行为。

```rust
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&numbers));
}
```
这个函数 `largest` 可以找出任何实现了 `PartialOrd` 和 `Clone` 特性的数据类型的最大值。

## 总结

泛型是 Rust 强大功能的体现之一，它提供了编写灵活且类型安全的代码的能力。通过上述示例和场景，我们可以看到泛型如何在多种不同的情境中发挥作用，从简单的函数到复杂的数据结构。掌握泛型将有助于你编写更加干净、高效且可重用的 Rust 代码。