---
title: 7. Test
tags:
  - Rust
  - basic
  - test
---

# WTF Rust 极简入门: Rust 测试

推特：[@0xAA_Science](https://twitter.com/0xAA_Science)

社区：[Discord](https://discord.gg/5akcruXrsk)｜[微信群](https://docs.google.com/forms/d/e/1FAIpQLSe4KGT8Sh6sJ7hedQRuIYirOoZK_85miz3dw7vA1-YjodgJ-A/viewform?usp=sf_link)｜[官网 wtf.academy](https://wtf.academy)

所有代码和教程开源在 github: [WTF-Rust](https://github.com/WTFAcademy/WTF-Rust)

## 简介

本文作者: [Eta](https://twitter.com/pwhattie)

本文主要介绍了 Rust 测试的函数、命令和种类, 干货满满。

- 测试函数: 测试中常用的宏、属性和枚举, 包括 `#[test]` 、`#[cfg(test)]`、 `panic!` 、`assert!` 、`assert_eq!`、`assert_ne!` 、
  `should_panic` 、`Result<T,E>` 。

- 测试命令 `cargo test`: 并行或连续的运行测试 `--test-threads`; 显示函数的输出 `--show-output`、运行单个测试、多个测试
  `cargo test 函数名` `cargo test --test 文件名`; 忽略部分测试 `#[ignore]` 、`--ignored` 。

- 测试种类: 单元测试(测试私有函数)、集成测试(tests 目录、共享子模块、二进制 binary crate)

## 1. 测试函数

测试函数是带有`test`属性标注的函数。使用`cargo test`命令运行测试时, 会调用标记了`test`属性的函数。使用 Cargo
新建一个库项目时，它会自动为我们生成一个测试模块和一个测试函数,`tests`模块还可以包括非测试的函数(无 test 属性的函数)
来进行常见操作, 可以添加任意数量的 test 模块或函数。

<details><summary>Tips: 安装 Rust</summary>
在 Linux 或 macOS 上安装 rustup

```shell
$ curl - - proto ' = https' - - tlsv1.2 https: //sh.rustup.rs -sSf | sh
```

在 Windows 上, 访问[安装页面](https://www.rust-lang.org/tools/install) 页面并按照说明安装 Rust。
更新和卸载、以及查看是否正确安装了 Rust

```shell
$ rustup update
$ rustup self uninstall
$ rustc - - version
```

安装 Rust, 具体参见[安装指南](https://doc.rust-lang.org/book/ch01-01-installation.html);

</details>

<details><summary>Tips: Rust 属性</summary>
Rust 的属性（attribute）是关于 Rust 代码片段的元数据, 它不会改变被它修饰的代码的逻辑,只是对代码进行修饰或标注, 类似于其他编程语言中的注解（annotations）或元数据（metadata）, 包括内部属性  `#![Attr]` 和外部属性 `#[Attr]` 。
根据用途可分为: 条件编译属性, 如 `#[cfg]` 和  `#[cfg_attr]` ; 用于 crate 的属性, 如  `#![no_std]`; 函数和模块的属性, 如 `#[test]` 用于标记测试函数，自动实现  Debug trait 即打印出调试信息的属性 `#[derive(Debug)]`; 和配置外部工具的属性 rustfmt 和  clippy。

具体更多信息参见[Rust 参考手册](https://rustwiki.org/en/reference/attributes.html)的 Attributes 部分。

</details>

### 1.1 创建一个新的库项目  `adder`

```shell
$ cargo new adder - - lib
Created library `adder` project
$ cd adder
```

文件名: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```

`cargo test`命令构建一个 Test Runner 可执行文件, 它会运行项目中所有的测试函数。测试函数的名称 `exploration` 和测试的运行结果
`ok`。`1 passed; 0 failed; 0 ignored; 0 filtered out;`表示通过、失败、忽略和过滤的测试数量, `0 measured`
统计是针对[性能测试](https://doc.rust-lang.org/unstable-book/library-features/test.html), `Doc-tests adder`是所有文档注释的测试结果,
确保文档和实际代码同步。

```shell
$ cargo test
Compiling adder v0.1.0 (file : ///projects/adder)
Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
Running target / debug / deps/ adder - ce99bcc2479f4607

running 1 test
test tests :: it_works ... ok

test result : ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc - tests adder

running 0 tests

test result : ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

```

> 注意: 后面的示例代码全部需要点击展开才能查看

### 1.2 测试中常用的宏、属性和枚举

#### 1.2.1 `panic!` 宏

测试失败的例子: 测试函数触发 panic 时, 测试会失败, 触发 panic 最简单的办法就是使用 `panic!`
宏。每一个测试都在一个新线程中运行，当主线程发现测试线程异常了，就将对应测试标记为失败, 列举测试失败的详细原因,
并且列出所有失败的测试。

<details><summary>点我展开示例代码</summary>

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

```

```text
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

- - - - tests::another stdout - - - -
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10: 9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed

```

</details>

#### 1.2.2 断言 (Assert) 宏: `assert!` 、`assert_eq!`  和  `assert_ne!` 检查代码是否按照期望返回正确的值

- `assert!` 宏

标准库的断言 `assert!`宏用于检查代码, 参数为布尔值。如果值为 true, 测试通过。如果值为 false, `assert!`会调用`panic!`宏,
测试失败。使用 glob 全局导入外部模块所有内容 `use super::*;`，以便在`tests`模块中使用所有在外部模块定义的内容。

  <details><summary>点我展开示例代码</summary>

  ```rust
  #[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

  ```

  ```rust
  #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
  ```

  ```shell
  running 2 tests
  test tests::smaller_cannot_hold_larger ... ok
  test tests::larger_can_hold_smaller ... ok

  test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

  </details>

- `assert_eq!`和`assert_ne!`

`assert_eq!`和`assert_ne!` 用于检查测试代码的值与期望值是否相等, 相当于 `assert!`宏的参数是 `==`或 `!=`
运算符的表达式。区别是断言失败时他们会打印出这两个具体的值，而`assert!`只会打印出`false`值。`assert_eq!`
宏在传递给它的两个值相等时通过，不相等时失败, `assert_ne!`宏则与之相反。
注意: 在一些语言和测试框架中，断言两个值相等的函数的参数叫做`expected`和`actual`，而且指定参数的顺序是很关键的。但是在
Rust 中，他们叫做`left`和`right`，同时指定参数的顺序并不重要。
`assert_eq!`和`assert_ne!`宏在底层分别使用了`==`和`!=`。当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了
`PartialEq`和`Debug`trait。所有的基本类型和大部分标准库类型都实现了这两个派生的 trait。自定义的结构体和枚举并没有实现这些
trait ，所以需要添加`#[derive(PartialEq, Debug)]`标注,
参见[“可派生的 trait”](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)。

  <details><summary>点我展开示例代码</summary>

  ```rust
  pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

  ```

  ```shell
  running 1 test
  test tests::it_adds_two ... ok

  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  ```

</details>

- 自定义失败信息

  自定义信息可以作为参数传递给`assert!` 、`assert_eq!`和`assert_ne!` 。`String`的大小可以增加，其内容也可以改变， 使用`+`
  运算符或`format!`宏来拼接`String`值。为测试函数增加一个自定义失败信息参数：带`{}`占位符的格式字符串，以及`greeting`函数的值。

  <details><summary>点我展开示例代码</summary>

  ```rust
  pub fn greeting(name: &str) -> String {
      format!("Hello {}!", name)
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn greeting_contains_name() {
          let result = greeting("Carol");
          assert!(result.contains("Carol"));
      }
  }
  ```

  ```rust
  pub fn greeting(_name: &str) -> String {
      String::from("Hello!")
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn greeting_contains_name() {
          let result = greeting("Carol");
      assert!(
          result.contains("Carol"),
          "Greeting did not contain name, value was `{}`", result
          );
      }
  }

  ```

  ```text
  ---- tests::greeting_contains_name stdout ----
  thread 'tests::greeting_contains_name' panicked at 'Greeting did not
  contain name, value was `Hello!`', src/lib.rs:12:9
  note: Run with `RUST_BACKTRACE=1` for a backtrace.

  ```

  </details>

#### 1.2.3 `should_panic` 属性

检查代码是否按照期望处理错误。这个属性在函数中的代码 panic 时会通过，没有 panic 时失败。`should_panic`测试结果只是告诉我们代码产生了
panic, 甚至在一些不是我们期望的原因而导致 panic 时也会通过。可选的`expected`参数使`should_panic`测试结果更精确,
确保错误信息中包含其提供的内容, 也就是说如果触发了 panic 但是 panic 的文字不包含 expected 参数里面的内容时测试仍然失败。

  <details><summary>点我展开示例代码</summary>

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

```

```shell
$ cargo test
Compiling adder v0.1.0 ( / Users/panwei/Desktop/code/Youtube/Rust/projects/11test / project/adder)
Finished test [unoptimized + debuginfo] target(s) in 0.32s
Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

- - - - tests::greater_than_100 stdout - - - -
note: test did not panic as expected

failures:
tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

```rust
// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(0);
    }
}

```

```shell
$ cargo test
Compiling adder v0.1.0 ( / Users/panwei/Desktop/code/Youtube/Rust/projects/11test / project/adder)
Finished test [unoptimized + debuginfo] target(s) in 6.26s
Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

- - - - tests::greater_than_100 stdout - - - -
thread 'tests::greater_than_100' panicked at src/lib.rs:9: 13:
Guess value must be greater than or equal to 1, got 0.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
panic message: `"Guess value must be greater than or equal to 1, got 0."`,
expected substring: `"Guess value must be less than or equal to 100"`

failures:
tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass ` - - lib`
```

  </details>

#### 1.2.4 `Result<T,E>`枚举

前面测试运行失败都是触发了 panic, 还可以使用 `Result<T,E>` 作为测试函数的返回类型来实现测试失败的目的。 不同于调用
`assert_eq!`宏，`Result<T,E>`是返回 Ok, 测试通过, 返回 Err, 测试失败。不能在使用`Result<T, E>`的测试中使用
`#[should_panic]`注解, 因为在测试失败时会返回 Err, 不会触发 panic。

<details><summary>点我展开示例代码</summary>

```rust

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

```

```shell

$ cargo test
Compiling adder v0.1.0 ( / Users/panwei/Desktop/code/Youtube/Rust/projects/11test / project/adder)
Finished test [unoptimized + debuginfo] target(s) in 1.46s
Running unittests src/lib.rs (target/debug/deps/adder-0980e06b8dfc4e08)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

</details>

## 2. 控制测试的运行: 测试命令 `cargo test` 的参数

`cargo run`会编译代码并运行生成的二进制文件, `cargo test`也会在测试模式下编译代码并运行生成的测试二进制文件。可以通过添加命令行参数来控制
`cargo test` 测试的运行, 将一部分命令行参数传递给`cargo test`，接着是分隔符`--`，再将另外一部分传递给生成的测试二进制文件。运行
`cargo test --help`会提示`cargo test`的有关参数，而运行`cargo test -- --help`可以提示在分隔符`--`之后使用的有关参数。

<details><summary>点我展开示例代码</summary>

```shell
$ cargo test - - help
$ cargo test - - - - help
```

</details>

### 2.1 并行或连续的运行测试: `--test-threads`

当运行多个测试时， Rust 默认使用线程来并行运行, 因为运行更快。并行运行需要确保每一个测试读写不同的文件,
测试不相互依赖，或不依赖任何共享的状态，否则一个测试可能会在另一个测试读写文件过程中修改了文件,
导致相互干扰。如果不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递`--test-threads`参数和希望使用线程的数量给测试二进制文件。

<details><summary>点我展开示例代码</summary>

```shell
$ cargo test - - - - test-threads=1
```

</details>

### 2.2 显示函数的输出: `--show-output`

Rust 默认捕获(不显示)所有输出, 比如 `println!`这类宏的输出在测试通过时不显示, 测试失败时才显示所有标准输出和其他错误信息。通过在末尾增加
`--show-output`参数来告知 Rust 显示通过测试的输出。

<details><summary>点我展开示例代码</summary>

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

```shell
$ cargo test - - - - show-output
```

</details>

### 2.3 运行部分测试: 单个测试、多个测试

通过向`cargo test`传递测试名称的参数来选择运行哪些测试。

运行单个测试: 通过向`cargo test`传递任意测试的名称来只运行这个测试,`2 filtered out`表明 2 个测试被过滤掉了。

运行多个测试: 指定部分测试的名称，任何匹配这个名称的测试会被运行, 可以通过模块名来运行一个模块中的所有测试。比如,
运行了所有名字中带有`add`的测试。

<details><summary>点我展开示例代码</summary>

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

```shell
// 运行单个测试
$ cargo test one_hundred
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out

```

```shell
// 运行多个测试
$ cargo test add
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

</details>

### 2.4 忽略部分测试: `#[ignore]` 、`--ignored`

对于不想运行的测试，可以在`#[test]`之后增加`#[ignore]`。`expensive_test`被列为`ignored`，没有运行。

如果只希望运行被忽略的测试，可以使用`cargo test -- --ignored`。

<details><summary>点我展开示例代码</summary>

```rust

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}
```

```shell
$ cargo test
Compiling adder v0.1.0 (file:///projects/adder)
Finished dev [unoptimized + debuginfo] target(s) in 0.24 secs
Running target/debug/deps/adder-ce99bcc2479f4607

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```

```shell
$ cargo test - - - - ignored
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

</details>

## 3. 测试的种类: 单元测试、集成测试

- 单元测试更小更集中，在隔离的环境中一次测试一个模块，或者测试私有接口, 与被测试的代码放在 *src*目录下的相同文件中, 并分别使用
  `tests`和`cfg(test)` 属性标注函数和模块, 前面介绍的就是单元测试。
- 集成测试完全位于被测试库的外部, 调用方式与其他使用这个库的代码一样，只测试公有接口而且每个测试可能会测试多个模块,
  需要创建一个 *tests*目录, 与被测试代码位于不同的文件夹, 只有运行 `cargo test` 时才会编译 tests 目录下的文件, 所以不需要
  `#[cfg(test)]`标注。集成测试的目的是检查被测试库的多个部分是否能一起正常运行,
  因为一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也很重要。

### 3.1 单元测试 Uint Tests

- 测试模块和 `#[cfg(test)]`
  创建项目会自动生成的测试模块。`cfg`属性代表*configuration(配置)*，告诉 Rust 下面的代码只被包含在特定的配置选项中,
  这里的配置选项是用来编译和运行测试的`test`，所以测试模块的`#[cfg(test)]`标注告诉 Rust 只在执行`cargo test`时才编译和运行模块中的
  `helper` 函数和 `#[test]` 标注的函数，而在运行`cargo build`时它们不应该被包含进编译结果中。

  <details><summary>点我展开示例代码</summary>

  ```rust

  #[cfg(test)]
  mod tests {
      #[test]
      fn it_works() {
          assert_eq!(2 + 2, 4);
      }
  }
  ```

  </details>

- 测试私有函数
  在其他语言中想要测试私有函数是一件困难的，甚至是不可能的事。但是 Rust 的私有性规则允许测试私有函数。

  <details><summary>点我展开示例代码</summary>

  ```rust
  pub fn add_two2(a: i32) -> i32 {
      internal_adder(a, 2)
  }

  fn internal_adder(a: i32, b: i32) -> i32 {
      a + b
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn internal() {
          assert_eq!(4, internal_adder(2, 2));
      }
  }
  ```

  </details>

### 3.2 集成测试 Integration Tests

#### 3.2.1 tests 目录

为了编写集成测试，需要在项目根目录创建一个*tests*目录，与*src*同级。接着可以随意在这个目录中创建任意多的测试文件。
保留测试私有函数中*src/lib.rs*的代码。并创建一个*tests*目录，新建一个文件*tests/integration_test.rs*，如下所示:

<details><summary>点我展开示例代码</summary>

```rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two2(2));
}
```

</details>

运行`cargo test`后, 得到三个部分的输出：单元测试、集成测试和文档测试。
第一部分单元测试与之前的一样：每个单元测试一行，接着是一个单元测试的摘要行。
集成测试部分以行`Running tests/integration_test.rs (target/debug/deps/integration-test-ce99bcc2479f4607)`
（在输出最后的哈希值可能不同）开头。接下来每一行是一个集成测试中的测试函数，以及一个位于`Doc-tests adder`部分之前的集成测试的摘要行。

<details><summary>点我展开示例代码</summary>

```shell
$ cargo test
Compiling adder v0.1.0 (file : ///projects/adder)
Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
Running target / debug / deps/ adder - abcabcabc

running 1 test
test tests :: internal ... ok

test result : ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running target / debug/ deps / integration_test - ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result : ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc - tests adder

running 0 tests

test result : ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

同样, 可以运行部分集成测试。

- 运行一个特定的集成测试: `cargo test 函数名`;
- 运行一个测试文件类的所有测试: `cargo test --test 文件名`。

<details><summary>点我展开示例代码</summary>

```shell
$ cargo test - - test integration_test
Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

#### 3.2.2 共享子模块

Cargo 会将每一个文件当作单独的 crate 来编译, 这些文件不共享行为 (与 src 下的文件规则不同), 所以需要在每一个文件中导入被测试库
`use adder` 。但是, 如果想创建一个 `helper` 帮助函数, 比如创建一个 *tests/common.rs*文件和一个名叫`setup`
的函数，并希望这个函数能被多个测试文件的测试函数调用。

<details><summary>点我展开示例代码</summary>

```rust
pub fn setup() {
    // 编写特定库测试所需的代码
}
```

</details>

再次运行测试，将会在测试结果中看到一个新的对应`common.rs`文件的测试结果部分，即便这个文件并没有包含任何测试函数，也没有任何地方调用了
`setup`函数。

<details><summary>点我展开示例代码</summary>

```shell
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

如果, 不希望`common`出现在测试输出中，就需要创建*`tests/common/mod.rs`*，而不是创建*`tests/common.rs`*。*`tests`*
目录中的子目录不会被作为单独的 `crate` 编译或作为一个测试结果部分出现在测试输出中。然后, 可以将 _`tests/common/mod.rs`_
作为模块在任何集成测试文件中使用。比如在 *`tests/integration_test.rs`*中的`it_adds_two`测试, 声明模块 `mod common` , 调用
`setup`函数 `common::setup()`。

<details><summary>点我展开示例代码</summary>

```rust
use adder;

mod common;

#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, adder : : add_two(2));
}
```

</details>

#### 3.2.3 二进制 binary crate

如果项目是二进制 binary crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 use
语句导入 src/main.rs 中定义的函数。只有库 library crate 才会向其他 crate 暴露可供调用和使用的函数, binary crate 是独立运行,
并且只会把核心逻辑代码即需要测试的代码放在 src/lib.rs 里。

## 总结

本文涵盖 Rust 测试常用函数 (assert!, panic! 等常用的宏、属性和枚举), 命令 (cargo test) 及种类 (单元测试、集成测试), 助你快速上手
Rust 测试。
