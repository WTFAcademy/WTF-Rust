---
title: Trait
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: Trait

在 Rust 中，trait 可以被认为是某种功能特性的集合，你可以将它看作是定义一组方法的方式，这些方法可以被多个类型实现。Trait 类似于其他语言中的接口，允许你定义共享的行为。这一节我们将深入理解 trait 的定义和使用，以及如何利用这个功能提升代码的模块化和可重用性。

## 定义 Trait

在 Rust 中定义 trait 类似于定义一个接口，您可以在其中声明一些方法，这些方法可以由多种数据类型共同实现。定义 trait 的语法如下：

```rust
trait Describable {
    fn describe(&self) -> String;

    fn default_description(&self) -> String {
        String::from("This is a default function")
    }
}
```

在这个例子中，我们使用了 `trait` 关键字来声明一个 trait。`Describable` 是 trait 的名字，里面包含了两个方法：
* `describe` 是一个必须由实现此 trait 的类型提供具体实现的方法。
* `default_description` 是一个默认方法，它不要求实现者提供自己的实现（除非他们想要自定义行为）。

## 实现 Trait

任何类型都可以实现一个或多个 trait。这意味着你可以为自定义类型添加方法，甚至为 Rust 标准库中的类型添加新的行为。

### 示例：定义一个结构体并实现 Trait

这里我们定义结构体 `Person`，并为它实现 `Describable` trait。

```rust
struct Person {
    name: String,
    age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", alice.describe());
}
```

在这里 `impl Describable for Person` 表示为 `Person` 结构体实现了 `Describable` trait。现在每个 `Person` 实例都可以使用 `describe` 方法。

## 使用 Trait 作为参数

Trait 还可以作为函数参数，这允许你编写更灵活和可复用的代码。

### impl Trait

`impl Trait` 指定函数参数可以是任何实现了特定 trait 的类型，适用于简单的情况。

```rust
fn output_description(item: &impl Describable) {
    println!("{}", item.describe());
}
```

`output_description` 函数接受任何实现了 `Describable` trait 的类型。对于 `item` 参数，我们指定了 `&impl` 关键字和 trait 的名称，这意味着你可以传递任何实现了该 trait 的实例的引用作为参数。这种方式避免了所有权的转移，因此可以多次使用同一个实例。

### Trait Bounds

对于接受多个 trait 的参数，或者对泛型参数有多个约束的情况，使用 trait bounds 语法，可以简化代码，特别是当约束变得复杂时。

```rust
fn some_function<T: Describable + Clone>(item: T) {
    let item_copy = item.clone();
    println!("Description: {}", item.describe());
}
```

函数 `some_function` 使用泛型参数 `T`，并通过 `T: Describable + Clone` 语法指定了多个 trait bounds。这意味着 `T` 必须同时实现 `Describable` 和 `Clone` 这两个 trait。使用 `+` 符号来链接多个 trait 是 Rust 的一种语法

## 实现 Trait 作为返回类型

你可以使用 `impl Trait` 语法在函数签名中指定返回值类型，这允许函数返回实现了指定 trait 的任何类型。

```rust
fn return_description() -> impl Describable {
    Person {
        name: String::from("Lisa"),
        age: 18,
    }
}
```

`return_description` 函数实现了返回了 `Describable` trait 类型。在这个例子中，我们选择返回 `Person` 实例。

## 继承 Trait

通过trait的继承，我们可以在现有 trait 的基础上构建更丰富的行为，同时确保它依赖的功能也得到实现。

```rust
trait Printable: Describable {
    fn print(&self) {
        println!("{}", self.describe());
    }
}

impl Printable for Person {}
```

在这个示例中, `Printable: Describable` 表示 `Printable` trait 继承自 `Describable`，并提供了一个默认实现的 `print` 方法。任何实现 `Printable` 的类型，也必须实现 `Describable`。

## 总结

Trait 是 Rust 中极其强大的一部分，它允许你定义和实现某些可以被多种类型共享的行为。通过学习如何定义、实现和使用 trait，你可以编写更具表现力、更可重用且更灵活的代码。Trait 也是 Rust 多态性和接口设计的基础，通过合理使用 trait，你可以提高代码的抽象级别和维护性！