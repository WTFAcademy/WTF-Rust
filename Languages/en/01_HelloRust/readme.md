---
title: 1. Hello Rust
tags:
  - Rust
  - Install
  - Cargo
  - wtfacademy
---

# WTF Rust Minimalist Introduction: 1. Hello Rust

I have recently studied `Rust`, consolidate the details, and also write a` WTF Rust minimalist in the door, for Xiaobai to use (programming big guys can find another tutorial), update 1-3 a week.

## Rust profile

`Rust` is a systemic programming language, developed by Mozilla Research, focusing on safety, speed and concurrency. It aims to help developers build a reliable and efficient software system while preventing common security vulnerabilities, such as empty pointer references, buffer overflow, etc.

The design concept of Rust` includes zero -cost abstraction, ensuring memory safety, incompetent competition, and pragmatism. It ensures the safety of memory through the mechanisms such as OWNERSHIP, borrowing, and life cycle (Lifetimes), while avoiding the performance expenses brought by garbage recycling.

## Install rust

First, we need to install `Rust` on your machine. `Rust` has a great installation tool, called` Rustup`, which will help us manage the version and the corresponding tool chain. Let's install it!

On `macOS`, `Linux`, or Unix-like operating systems, install using the following command:

Open your terminal (or command line) and enter the following command:

```Bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This command will download a script and execute it. The script will automatically install `rustup` and the default version of Rust (including `rustc`, the Rust compiler, and `cargo`, the Rust package manager). Follow the instructions in the terminal, and once everything is set up, we can proceed to the next step!

After installation is complete, you can check the Rust version by running the `rustc --version` command to verify if the installation was successful. If it indicates that `rustc` cannot be found, it is an environment variable issue. Check whether the directory `~/.cargo/bin` has been added to `PATH`. You can restart the terminal or manually source it to apply the environment changes.

If it is the `Windows` system, please refer to [Official Installation Instructions](https://forge.rust-lang.org/infra/other-installation-methods.html)

## Hello Rust program

In the journey of programming language, the program is the first step of the traditional. It is the simplest program that "ask well" to the world. It is no exception in Rust, let's try it!

1. Create a new folder, named `Hello_rust`, and then enter this folder.
2. In the `Hello_rust` folder, create a new file, named` main.rs`. The file name `.s` suffix represents a RUST source file.
3. Open the `main.rs`, use your favorite text editor, enter the following code:

`` `rust
fn main () {
 Println! ("Hello, Rust!");
}
`` `

To explain briefly, this code defines a function `main`, which is the entrance point of each Rust program. When the Rust program runs, it executes the code in the `main` function. `Println!` is a macro (we talk about macro in the future), used to output text to the terminal.

4. Save the file and return to the terminal to make sure you are in the `Hello_rust` folder, and then enter the following command to compile and run your program:

`` `Bash
Rustc Main.rs
./main
`` `

If you are a user, the command of the running program may be slightly different, such as entering the `main` directly.

If everything goes well, your terminal will output:

`` `
Hello, Rust!
`` `

Congratulations, you have successfully run your first Rust program!

## Use Cargo

In Rust's world, `Cargo` is your good friend. It is not only a bag management tool, but also helps you build projects, download dependencies, run tests, and so on. Let's take a look at how to use the `Cargo` to create and run a new project.

1. Open the terminal and enter the following command to create a new RUST project:

`` `Bash
Cargo new hello_cargo
`` `

This command will create a new folder called `Hello_cargo`, which contains a preliminary project structure.

2. Enter the `Hello_cargo` folder, you will find that there are two main files:` cargo.toml` and `src/main.rs`. `Cargo.toml` is your project configuration file, while` src/main.rs` is your main program file, which has the default `Hello, Rust!` Code.

3. Let's compile and run the project directly to see the magic of `Cargo`! In the terminal of the `hello_cargo` folder, enter the following command:

`` `Bash
Cargo run
`` `

`Cargo Run` command will automatically compile your code (if necessary) and run the generated program. You should see the greetings of `Hello, Rust!` In the terminal.


4. If you use Rustrover, you can directly run the program directly in the `Cargo` plug -in that comes with your own` Cargo` plug -in to facilitate the rapid verification of the program.

! [img.png] (imgs/img.png)

5. In the demonstration code in the subsequent chapters, I will use `Cargo` to demonstrate, so that everyone will run and test.

that's all! Now, you already know how to install the `Rust`, write and run the` Rust` program, and use the simple item to manage the `Cargo`. This is just the tip of the iceberg. The world of `Rust` is full of possibilities and adventures waiting for you. Are you ready? Let's move forward and go deep into the wonderful journey of `Rust`!

## Summary

This chapter mainly introduces the `Rust` installation method, write the first` Rust` program --`hello Rust`, and introduce how to use `Cargo` for project development








