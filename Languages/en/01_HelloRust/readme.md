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

Install the following commands on the following commands

Open your terminal (or command line) and enter the following command:

`` `Bash
Curl -ProTo '= https' --tlsv1.2 --ssf https://sh.rustup.rs | SH
`` `

This command will download a script and execute. The script will automatically install the default versions of `Rustup` and Rust (including` Rustc`, Rust's compiler, and `Cargo`, Rust's package management tool). According to the instructions in the terminal, after everything is proper, we can enter the next step!

After the installation is completed, you can check the RUST version by running the `Rustc -" command to verify whether the installation is successful. If you can't find the `Rustc`, it is an environmental variable problem. You can view the directory` ~/.cargo/bin` whether to add it to the `Path`. You can restart the terminal or manual source to make the environment take effect.

If it is the `Windows` system, please refer to [Official Installation Instructions]

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
