---
title: 2. Base Type
tags:
- Rust
- Variables
- wtfacademy
---

# WTF Rust Minimalist Introduction: Base Types

In this chapter, we will explore the basic syntax elements in Rust through examples with detailed explanations. We will cover variables and mutability, as well as data types (including scalar and compound types).

## Variables and Mutability

In Rust, variables are immutable by default. This means that once a value is assigned to a variable, you cannot change it. However, you can use the `mut` keyword to make a variable mutable.

### Immutable Variables

```rust
let x = 5;
println!("The value of x is: {}", x);
// x = 6; // This line will cause a compile error because x is immutable
```

### Mutable Variables

```rust
let mut y = 5;
println!("The value of y is: {}", y);
y = 6; // This is allowed because y is mutable
println!("The value of y is: {}", y);
```

## Data Types

Rust is a statically typed language, meaning all variable types must be known at compile time. The compiler usually infers types based on values and usage. Rust data types can be broadly divided into two categories: scalar and compound types.

### Scalar Types

Scalar types represent single values. As previously mentioned, Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

#### Integers

Integers are numbers without fractional parts. In Rust, integer types are divided into signed and unsigned. For example, the `i32` type represents a signed 32-bit integer (`i` stands for `integer`). The `u` prefix indicates an unsigned type. Here are the built-in integer types available in Rust:

| Length       | Signed Type | Unsigned Type |
|:------------:|:-----------:|:-------------:|
| 8-bit        | `i8`        | `u8`          |
| 16-bit       | `i16`       | `u16`         |
| 32-bit       | `i32`       | `u32`         |
| 64-bit       | `i64`       | `u64`         |
| 128-bit      | `i128`      | `u128`        |
| Architecture | `isize`     | `usize`       |

For signed types, the numeric range is from `-(2^(n-1))` to `2^(n-1) - 1`, where `n` is the number of bits. For example, `i8` can store values from `-128` to `127`. Unsigned types range from `0` to `2^n - 1`, so `u8` ranges from `0` to `255`.

The size of `isize` and `usize` depends on the architecture of the computer running the program: 32 bits for 32-bit CPUs and 64 bits for 64-bit CPUs.

Integer literals can be represented in several ways:

- Decimal: without a prefix, e.g., 98_222 (underscores improve readability)
- Hexadecimal: with a `0x` prefix, e.g., 0xff
- Octal: with a `0o` prefix, e.g., 0o77
- Binary: with a `0b` prefix, e.g., 0b1111_0000
- Byte (only for `u8` type): with a `b` prefix, e.g., b'A'

```rust
let x: i32 = -123;
let y: u32 = 123;
println!("x is {}, y is {}", x, y);
```

#### Floating-Point Numbers

Rust provides two floating-point types: single precision `f32` and double precision `f64`. By default, floating-point numbers are of type `f64` because it offers good precision and speed.

| Type | Precision            |
|:----:|:--------------------:|
| `f32`| Single precision (32-bit) |
| `f64`| Double precision (64-bit) |

```rust
let x = 2.0; // No type specified, inferred as f64 (default)
let y: f32 = 3.0; // Explicitly specified as f32
println!("x is {}, y is {}", x, y);

let x1 = 2.0; // No type specified, but will be inferred as f32 because of the next statement
let y1: f32 = 3.0; // Explicitly specified as f32
let z1 = x1 + y1;
println!("x1 is {}, y1 is {} z1 is {}", x1, y1, z1);
```

#### Booleans

```rust
let t = true;
let f: bool = false; // Explicit type annotation
println!("t is {}, f is {}", t, f);
```

#### Characters

Characters, encoded in Unicode, are 4 bytes in size in Rust:

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
println!("c is {}, z is {}, heart_eyed_cat is {}", c, z, heart_eyed_cat);
println!("The character 'c' occupies {} bytes in memory", std::mem::size_of_val(&c));
```

In Rust, the length of the `String` type depends on the encoding used. By default, Rust uses UTF-8 encoding, where each character occupies 1-4 bytes. The `char` type always occupies 4 bytes, even if some characters need only 1-3 bytes in specific encodings. The advantages are:
- Ensuring all `char` values occupy a fixed size in memory, aiding memory alignment and access efficiency.
- Avoiding the overhead of encoding conversion, directly using 4-byte values for efficient processing.
- Adequately representing all Unicode scalar values, ensuring future compatibility.

### Compound Types

Compound types can group multiple values into one type. Rust includes tuples and arrays as compound types.

#### Tuples

Tuples are a basic way to group multiple values of different types into one compound type with a fixed length.

```rust
let tup: (i32, f64, u8, char) = (-500, 6.4, 1, 'z');
let (w, x, y, z) = tup; // Destructuring the tuple
println!("The value of x is: {}", x);
```

#### Arrays

Arrays group multiple values of the same type. Unlike tuples, arrays have a fixed length.

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
println!("The first element is {}, the second element is {}", first, second);
```

## Summary

In this chapter, we covered Rust's basic data types: the four basic scalar types (integers, floating-point numbers, booleans, and characters) and the two compound types (tuples and arrays).
