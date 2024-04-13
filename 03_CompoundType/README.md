---
title: 3. Compound type
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust 极简入门: 复合类型扩展：元组、数组、字符串和切片
## 元组

元组是一个可以包含多个不同类型的值的复合类型。它非常有用，当你想要从一个函数返回多个值或者将多个值组合成一个集合时。

### 示例代码

```rust
fn main() {
    let mixed = ("Rust", 2023, 3.14, true);
    let (lang, year, pi, status) = mixed;

    println!("Language: {}, Year: {}, PI: {}, Status: {}", lang, year, pi, status);
}
```

## 数组

数组用于存储多个相同类型的元素。当你需要处理一组相同类型的数据时，数组就非常有用。

### 示例代码

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }

    // 访问指定位置的数据
    println!("First element: {}", numbers[0]);
    
    
}
```

## 字符串和切片

字符串是一种非常常见的复合类型，用于存储文本。在 Rust 中，有两种主要的字符串类型：`String` 和字符串切片 `&str`。

### 创建和使用字符串

```rust
fn main() {
    let mut s = String::from("Hello"); // 可变的String类型
    s.push_str(", world!"); // 修改String
    println!("{}", s);

    let slice = &s[0..5]; // 获取部分String作为切片
    println!("Slice: {}", slice);
}
```

字符串切片 `&str` 是对存储在某处（通常为String类型）的 UTF-8 编码字符串数据的引用。它们是不可变的。当你想要部分引用String中的内容或传递小量数据时，切片特别有用。

## 切片

切片允许你引用集合中的一段连续元素序列，而不是整个集合。它们同样适用于数组。

### 示例代码

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // 引用数组的一部分
    
    for &item in slice.iter() {
        println!("{}", item);
    }
}
```

在这里，`slice` 是一个指向 `numbers` 数组中三个元素的切片。切片是一种非常有用的工具，因为它允许你安全地访问数组或字符串的部分元素，而无需复制它们。

## 小结

通过扩展探讨元组、数组、字符串及切片，我们更全面地了解了 Rust 中复合类型的使用和优势。这些结构提供了处理数据和组织程序逻辑方面的灵活性和效率。希望这些示例帮助你更好地理解这些概念，并能够在你的 Rust 项目中有效地使用它们。