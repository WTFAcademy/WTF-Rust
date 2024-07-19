---
title: 4. Structs and Enums
tags:
- Rust
- basic
- wtfacademy
---

# WTF Rust Minimalist Introduction: 4. Structs and Enums

## 1. Defining and Using Structs

First, let's talk about structs. A struct is like a custom data type used to represent entities in the real world, such as people, animals, or anything else we want to create. For example, to describe a person with attributes like age, name, and address, we can define a struct. It looks like this:

```rust
// Define a Person struct
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    // Create a Person instance with the name Alice and age 30
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        address: String::from("China"),
    };

    // Output the information of this Person instance
    println!("{} is {} years old.", person1.name, person1.age);
}
```

## 2. Method Definitions

Now, let's discuss methods, which are functions associated with structs. The difference between a `method` and a `function` is that a `method` has an **owner**, with its first parameter typically being `&self`, indicating the owner. When calling a method, you must specify the owner, like `owner.method()`. In contrast, a `function` has no owner and is considered a public function. The same program cannot have two functions with the same signature. Methods, having owners, allow different owners to have methods with the same signature. By using methods, we can provide behavior for structs. It sounds advanced but is actually simple:

```rust
impl Person {
    // Define a method to display personal information
    fn show_info(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Bob"),
        age: 25,
    };

    // Call this method to display personal information
    person1.show_info();
}
```

## 3. Enums and Pattern Matching

Next, let's talk about enums and pattern matching. Enums allow us to define various possible data variants, and then handle these variants through pattern matching. It sounds complex but is quite interesting:

```rust
// Define an enum to represent different colors
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Blue;

    // Use pattern matching to handle different colors
    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }
}
```

By using structs and enums, we can better organize and manage data, and pattern matching allows us to handle data more flexibly. I hope this chapter helps you gain a clear understanding of structs and enums!
