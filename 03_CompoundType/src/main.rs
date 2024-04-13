use itertools::Itertools;

fn main() {
    let mixed = ("Rust", 2023, 3.14, true);
    let (lang, year, pi, status) = mixed;

    println!("Language: {}, Year: {}, PI: {}, Status: {}", lang, year, pi, status);


    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }

    // 访问指定位置的数据
    println!("First element: {}", numbers[0]);


    let mut s = String::from("Hello"); // 可变的String类型
    s.push_str(", world!"); // 修改String
    println!("{}", s);

    let slice = &s[0..5]; // 获取部分String作为切片
    println!("Slice: {}", slice);


    let numbers = [1, 2, 3, 4, 5];
    // 给数组 numbers 增加数字 6和7

    let slice = &numbers[0..2]; // 引用数组的一部分

    for &item in slice.iter() {
        println!("{}", item);
    }
    slice.iter().for_each(|&item| println!("{}", item));
}
