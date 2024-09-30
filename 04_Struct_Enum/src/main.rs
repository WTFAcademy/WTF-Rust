// 定义一个Person结构体
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 定义一个方法，展示个人信息
    fn show_info(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}

fn show_info() {
    println!("this is a function");
}

fn main() {
    // 创建一个名为 Alice、年龄为30的Person实例    
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 调用这个方法来展示个人信息
    person1.show_info();

    // 调用这个函数，不需要 owner
    show_info();
}
