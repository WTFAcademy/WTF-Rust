---
title: 5. Ownership
tags:
  - Rust
  - basic
  - wtfacademy
---

# WTF Rust Minimalist Introduction: Ownership, Borrowing, and References
This chapter covers the essence of `Rust`: ownership, borrowing, and references. These concepts are the foundation of `Rust`'s memory safety, helping developers write safe and efficient code while avoiding common errors found in traditional languages, such as null pointer access and data races.

## 1. Ownership Rules

In `Rust`, the core rules of the ownership system can be summarized as follows:

- Every value has a single owner variable.
- A value can only have one owner at a time.
- When the owner goes out of scope, the value is dropped.

These rules ensure memory safety and prevent leaks while avoiding manual memory management.

### Example: Ownership Transfer

```rust
fn main() {
    let s1 = String::from("hello");  // s1 is the owner of the "hello" object

    let s2 = s1;                     // Ownership is transferred from s1 to s2; s1 becomes unusable
    // println!("{s1}");             // Error: s1 no longer holds the string

    display(s2);                    // s2 transfers ownership to the function parameter s; s2 becomes unusable
    // println!("{s2}");             // Error: s2 is no longer usable
}

fn display(s: String) {
    println!("{:?}", s);
}
```

## 2. Borrowing

In `Rust`, borrowing refers to obtaining **access rights** to data via references, rather than **ownership**, denoted by the `&` symbol. Borrowing allows multiple parts of code to access the same data simultaneously without transferring ownership. Borrowing comes in two forms: immutable borrowing and mutable borrowing.

- **Immutable Borrowing**: Allows multiple borrows, but the data cannot be modified during the borrowing period; `Rust` defaults to immutable borrowing.
- **Mutable Borrowing**: Allows data modification, but only one mutable borrow is allowed at a time.

### Example: Immutable Borrowing

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 is immutably borrowed; the function can only read but not modify s1

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {  // s is a reference to s1
    s.len()
}
```

### Example: Mutable Borrowing

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);  // s is mutably borrowed; the function can modify s

    println!("{}", s);
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(", wtf!");
    some_string
}
```

## Summary

Through these examples, you should have a basic understanding of `Rust`'s core concepts: ownership and borrowing. Mastering these concepts is crucial for effectively utilizing `Rust`'s features, enabling you to write safer and more efficient code. If you have any questions or need further clarification, feel free to ask!
