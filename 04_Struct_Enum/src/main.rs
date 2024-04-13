
// 定义一个Person结构体
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // 创建一个名为Alice、年龄为30的Person实例
    let mut person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    person1.name = String::from("Alice22");

    // 输出这个Person实例的信息
    println!("{} is {} years old.", person1.name, person1.age);

}