
fn calculate_length(s: &String) -> usize {  // s 是对 s1 的引用
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main(){
    let s1 = String::from("hello");  // s1 成为所有者
    let s2 = s1;                     // 所有权从 s1 转移至 s2

    // println!("{s1}");             // 错误：s1 不再持有字符串
    println!("{s2}");


    //不可变借用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // s1 被不可变借用

    println!("The length of '{}' is {}.", s1, len);


    //可变借用
    let mut s = String::from("hello");

    change(&mut s);  // s 被可变借用

    println!("{}", s);



}

