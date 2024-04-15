
fn borrow<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}

fn longest<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct Book<'a> {
    title: &'a str,
    pages: i32,
}

fn main() {
    // 示例：使用 borrow 函数
    let num1 = 34;
    let num2 = 50;
    let larger_number = borrow(&num1, &num2);
    println!("The larger number is {}", larger_number);

    // 示例：使用 longest 函数
    let string1 = "He!";
    let string2 = "Hi!";
    let longer_string = longest(string1, string2);
    println!("The longest string is {}", longer_string);


    let title = String::from("Rust Programming");
    let book = Book {
        title: &title,
        pages: 384,
    };

    println!("Book: {} - {} pages", book.title, book.pages);
}
