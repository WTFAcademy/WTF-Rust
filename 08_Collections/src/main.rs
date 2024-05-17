use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();  // 创建一个空的向量
    v.push(5);                        // 向向量中添加元素
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {:?}", v);       // 打印向量


    let v = vec![1, 2, 3, 4, 5];       // 使用宏创建并初始化向量
    println!("Vector: {:?}", v);

    for i in &v {
        println!("{}", i);
    }


    let mut s = String::new();        // 创建一个空字符串
    s.push_str("Hello");              // 添加文本
    s.push(' ');                      // 添加单个字符
    s.push_str("world!");

    println!("String: {}", s);


    let s1 = "Hello".to_string();
    let s2 = " World!";
    let s3 = s1 + s2;  // 注意：s1 在这里被移动了，不能再被使用

    println!("Combined string: {}", s3);

    let s1: &str = "Hello, world!"; // 字符串字面量的类型是 &str
    let s2: &str = &s1[0..5]; // 使用切片语法创建一个子字符串
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    
    let s3: String = s1.to_string(); // 将 &str 转换为 String
    println!("s3: {}", s3);
    
    print_str(s1); // 将 &str 传递给函数
    
    

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    if let Some(score) = scores.get("Blue") {
        println!("Blue team's score: {}", score);
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn print_str(s: &str) {
    println!("Inside print_str: {}", s);
}
