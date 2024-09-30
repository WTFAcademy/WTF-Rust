fn display(s: String) {
    println!("{:?}", s);
}

fn calculate_length(s: &String) -> usize {
    // s 是对 s1 的引用
    s.len()
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(", wtf!");
    some_string
}

fn main() {
    let mut s1 = String::from("hello"); // s1 是 hello 对象的所有者
    s1.push_str(", wtf!"); // s1 可以修改 hello 对象，追加字符串

    let s2 = s1; // 所有权从 s1 转移至 s2
    // println!("{s1}"); // 错误：s1 不再拥有字符串

    display(s2); //s2 将所有权转让给函数参数s，s2将变得不可用
    // println!("{s2}"); // 错误：s2不可用

    //不可变借用
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // s1 发生不可变借用，函数只能读取但不能修改 s1
    println!("The length of '{}' is {}.", s3, len);

    //可变借用
    let mut s4 = String::from("hello");
    change(&mut s4); // s4 发生可变借用，函数可以修改 s4
    println!("{}", s4);
}
