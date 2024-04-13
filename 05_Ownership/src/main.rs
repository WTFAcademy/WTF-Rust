fn main(){
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);


    // 泛型结构体示例
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("Point coordinates: ({}, {})", integer_and_float.x, integer_and_float.y);


    //Trait示例
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    struct Article {
        title: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{} - {}", self.title, &self.content[..10])
        }
    }

    let article = Article {
        title: "Rust Language".to_string(),
        content: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
    };

    println!("New article available! {}", article.summarize());

    //生命周期示例：函数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

}