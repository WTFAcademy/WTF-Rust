---
title: 3. Base Type
tags:
- Rust
- basic
- wtfacademy
---

# WTF Solidity极简入门: 基础类型

在这一章节中，我们将通过具有详细说明的实例，探索 Rust 中的基础语法元素。我们会覆盖变量和可变性、数据类型（包括标量和复合类型）。

## 变量和可变性

在 Rust 中，变量默认是不可变的。这意味着一旦值被赋予一个变量，你就不能更改这个值。不过，你可以使用 `mut` 关键字来使变量可变。

### 不可变变量

```rust
let x = 5;
println!("The value of x is: {}", x);
// x = 6; // 这行会导致编译错误，因为 x 不可变
```

### 可变变量

```rust
let mut y = 5;
println!("The value of y is: {}", y);
y = 6; // 这是允许的，因为 y 是可变的
println!("The value of y is: {}", y);
```

## 数据类型

Rust 是静态类型语言，意味着所有的变量类型都必须在编译时已知。Rust 数据类型大致可以分为两大类：标量和复合类型。

### 标量类型

标量类型代表单一值。如前面所提到的，Rust 有四种基本的标量类型：整数、浮点数、布尔值和字符。

#### 整数
整数是没有小数部分的数字。在Rust中，整数类型分为有符号和无符号两种。例如，你之前见过的`i32`类型代表有符号的32位整数（`i`是英文单词`integer`的首字母，意味着整数）。与之对应的是`u`前缀，代表无符号（`unsigned`）类型。以下是Rust中可用的内置整数类型：

| 长度         | 有符号类型 | 无符号类型 |
|:------------:|:-----------:|:-----------:|
| 8位         | `i8`        | `u8`        |
| 16位         | `i16`       | `u16`       |
| 32位         | `i32`       | `u32`       |
| 64位         | `i64`       | `u64`       |
| 128位        | `i128`      | `u128`      |
| 视架构而定   | `isize`     | `usize`     |

对于有符号类型，其规定的数字范围是`-(2^(n-1))`到`2^(n-1) - 1`，其中`n`是位数。例如，`i8`可以存储的数字范围从`-128`到`127`。而无符号类型的范围是从`0`到`2^n - 1`，因此`u8`的范围是从`0`到`255`。

至于`isize`和`usize`，它们的大小取决于运行程序的计算机CPU类型：如果CPU是32位的，它们就是32位；如果CPU是64位的，那么它们就是64位。

```rust
let x: i32 = -123;
let y: u32 = 123;
println!("x is {}, y is {}", x, y);
```

#### 浮点数
Rust提供了两种精度的浮点类型：单精度`f32`和双精度`f64`。默认情况下，浮点数类型是`f64`，因为它同时提供了很好的精度和速度。

| 类型  | 精度                  |
|:-----:|:--------------------:|
| `f32` | 单精度（32位）浮点数 |
| `f64` | 双精度（64位）浮点数 |

```rust
let x = 2.0; // 默认为 f64
let y: f32 = 3.0; // 显式声明为 f32
println!("x is {}, y is {}", x, y);
```

#### 布尔值

```rust
let t = true;
let f: bool = false; // 显式类型声明
println!("t is {}, f is {}", t, f);
```

#### 字符
字符，由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
println!("c is {}, z is {}, heart_eyed_cat is {}", c, z, heart_eyed_cat);
println!("字符'c'占用了{}字节的内存大小",std::mem::size_of_val(&c));
```

### 复合类型

复合类型可以将多个值组成一个类型。Rust 主要包括元组（tuple）和数组（array）两种复合类型。


#### 元组

```rust
let tup: (i32, f64, u8, char) = (-500, 6.4, 1, 'z');
let (w, x, y, z) = tup; // 解构元组
println!("The value of x is: {}", x);
```

#### 数组

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
println!("The first element is {}, the second element is {}", first, second);
```
