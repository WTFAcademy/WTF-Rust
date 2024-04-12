# 函数
---
title: 3. Function
tags:
- Rust
- basic
- wtfacademy
---

## 函数

在Rust中，函数是执行特定任务的代码单元。函数的使用可以增加代码的可重用性、简化复杂的任务，并有助于对代码进行模块化管理。

### 函数定义

要定义一个函数，你需要使用`fn`关键字，后跟函数名、参数列表以及函数体。

```rust
fn greet() {
    println!("Hello, world!");
}
```

### 带参数的函数

函数可以接受参数。你需要在函数名后的括号内声明参数和它们的类型。

```rust
fn print_sum(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}
```

### 返回值

函数可以返回值。返回值的类型需要在箭头`->`后指定。在Rust中，函数的最后一个表达式会被用作返回值，或者你可以使用`return`关键字明确返回值。

```rust
fn add_two(a: i32) -> i32 {
    a + 2
    // 或 `return a + 2;`
}
```

## 控制流

Rust提供了多种控制程序执行流程的方式，包括`if`表达式和循环（`loop`、`while`、`for`）。

### If 表达式

`if`表达式让你根据条件执行不同的代码分支。

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

### 循环

Rust提供了几种循环方式：`loop`、`while`和`for`。

- **Loop**

`loop`关键字可以创建一个无限循环。

```rust
loop {
    println!("again!");
    break; // 无限循环，但这里我们立刻跳出循环
}
```

- **While**

`while`循环在其条件为真时循环执行。

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
println!("LIFTOFF!!!");
```

- **For**

`for`循环用于遍历一个集合，如数组或范围。

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}
```

通过了解函数的定义与使用以及掌握Rust的控制流机制，你将能够编写出结构清晰、逻辑严谨的Rust程序。函数和控制流是Rust中最基本的构建块之一，熟练使用它们将大大提升你的Rust编程技能。