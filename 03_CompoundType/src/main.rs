fn main() {
    let mixed = ("Rust", 2023, 3.14, true);
    let (lang, year, pi, status) = mixed;

    println!("Language: {}, Year: {}, PI: {}, Status: {}", lang, year, pi, status);


    // 1.显式声明动态数组类型
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("vector v1: {:?}", &v1);

    // 2.使用宏 vec! 来创建数组，支持在创建时就给予初始化值
    let v2 = vec![1u8, 2, 3];
    println!("vector v2: {:?}", &v2);

    let mut s = String::from("Hello"); // 可变的String类型
    s.push_str(", world!"); // 修改String
    println!("{}", s);

    let slice = &s[0..5]; // 获取部分String作为切片，第一个参数是切片的起始索引（包含），第二个参数是切片的结束索引（不包含）
    println!("Slice: {}", slice); // Hello


    let numbers = [1, 2, 3, 4, 5];

    let slice = &numbers[0..2]; // 引用数组的一部分

    for &item in slice.iter() {
        println!("{}", item);
    }
    slice.iter().for_each(|&item| println!("{}", item));
}
