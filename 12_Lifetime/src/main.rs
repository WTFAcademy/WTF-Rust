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

    /*
    悬空引用
    let s2;
    {
        let s1 = String::from("hello");
        // `s1` does not live long enough, borrowed value does not live long enough
        s2 = &s1;
    }
    println!("s2: {}", s2);
     */


    let string1 = String::from("abcdefghijklmnopqrstuvwxyz");
    let result1;
    {
        let string2 = String::from("123456789");
        result1 = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result1);
    }

    // `string2` does not live long enough，borrowed value does not live long enough
    //  println!("The longest string is {}", result);


    let string3 = String::from("abcdefghijklmnopqrstuvwxyz");
    let result2;
    {
        let string4 = String::from("123456789");
        result2 = longest(string4.as_str(), string3.as_str());
        println!("The longest string is {}", result2);
    }

    // `string4` does not live long enough，borrowed value does not live long enough
    // println!("The longest string is {}", result2);
}
