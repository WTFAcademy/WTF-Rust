---
title: 3. Compound Type
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust Minimalist Introduction: Compound Types Expanded: Tuples, Arrays, Strings, and Slices

## Tuples

A tuple is a compound type that can contain multiple values of different types with a fixed length. Once defined, the length of a tuple cannot grow or shrink, and indices start from 0. Tuples are very useful when you want to return multiple values from a function or group multiple values into a single collection.

### Example Code

```rust
fn main() {
    let mixed = ("Rust", 2023, 3.14, true);
    let (lang, year, pi, status) = mixed;

    println!("Language: {}, Year: {}, PI: {}, Status: {}", lang, year, pi, status);
}
```

## Dynamic Arrays (Vectors)

Dynamic arrays, `Vec<T>`, are flexible data structures that allow for resizing at runtime. This means their length can be changed dynamically to add or remove elements as needed. They are particularly useful for handling an uncertain number of data elements, such as reading an unknown number of user inputs or dynamically generating datasets. Dynamic arrays use a generic parameter `T`, meaning they can store values of **any type**, such as integers, characters, or floating-point numbers discussed earlier, but once the type is specified, all elements in the array must be of the **same type**.

### Example Code

```rust
fn main() {
    // 1. Explicitly declare the type of the dynamic array
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("vector v1: {:?}", &v1);

    // 2. Use the macro vec! to create an array with initial values
    let v2 = vec![1u8, 2, 3];  
    println!("vector v2: {:?}", &v2);
}
```

## Strings and Slices

Strings are a very common compound type used to store text. In `Rust`, there are two main string types: `String` and string slices `&str`.

### Creating and Using Strings

```rust
fn main() {
    let mut s = String::from("Hello"); // Mutable String type
    s.push_str(", world!"); // Modify the String
    println!("{}", s);

    let slice = &s[0..5]; // Get a slice of the String
    println!("Slice: {}", slice);
}
```

A string slice `&str` is a reference to a `UTF-8` encoded string data stored somewhere (usually a `String` type) and is hardcoded into the program binary at compile time, making it immutable. Slices are particularly useful when you want to reference a portion of a `String` or pass a small amount of data.

## Slices

Slices allow you to reference a contiguous sequence of elements within a collection rather than the entire collection. They are also applicable to arrays.

### Example Code

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4]; // Reference a part of the array
    
    for &item in slice.iter() {
        println!("{}", item);
    }
}
```

Here, `slice` is a reference to a portion of the `numbers` array containing three elements. Slices are a very useful tool because they allow you to safely access a subset of elements from an array or string without copying them.

## Summary

By exploring tuples, arrays, strings, and slices, we gain a more comprehensive understanding of compound types in `Rust` and their advantages. These structures offer flexibility and efficiency in handling data and organizing program logic. Hopefully, these examples help you better understand these concepts and effectively use them in your `Rust` projects.
