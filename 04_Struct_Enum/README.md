---
title: 4. Struct Enum
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 4. 结构体，枚举类型

## 1. 定义和使用结构体

首先，让我们谈谈结构体。结构体就像是一种自定义数据类型，用来表示现实世界中的实体，比如人、动物或任何我们想创建的东西。看起来像这样：

```rust
// 定义一个Person结构体
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // 创建一个名为Alice、年龄为30的Person实例
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 输出这个Person实例的信息
    println!("{} is {} years old.", person1.name, person1.age);
}
```

## 2. 方法定义

好了，现在我们来谈谈方法，这是与结构体关联的功能。通过方法，我们可以为结构体提供行为。这听起来有点高级，但实际上很简单：

```rust
impl Person {
    // 定义一个方法，展示个人信息
    fn show_info(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    // 调用这个方法来展示个人信息
    person1.show_info();
}
```

## 3. 枚举和模式匹配

下面，让我们来谈谈枚举和模式匹配。枚举允许我们定义一些可能的数据变体，然后通过模式匹配来处理这些变体。这听起来复杂，但实际上很有趣：

```rust
// 定义一个枚举表示不同颜色
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Blue;

    // 使用模式匹配来处理不同颜色
    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }
}
```

通过结构体和枚举，我们能够更好地组织和管理数据，而模式匹配则可以让我们对数据进行更灵活的处理。希望这一章的内容能够让你对结构体和枚举有个清晰的了解！如果有任何问题或疑惑，请随时联系我。愿你写作顺利！