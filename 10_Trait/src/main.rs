// 定义一个 trait Describable，包含一个必须实现的方法和一个默认实现的方法
trait Describable {
    fn describe(&self) -> String;

    // 默认实现的方法，可以被实现该 trait 的类型覆盖
    fn default_description(&self) -> String {
        String::from("This is default functions")
    }
}

// 定义一个结构体 Person，并为其自动生成 Clone trait 的实现
#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

// 为结构体 Person 实现 Describable trait
impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

// 定义一个函数，接受任何实现了 Describable trait 的类型的引用，并输出描述
fn output_description(item: &impl Describable) {
    println!("{}", item.describe());
}

// 定义一个函数，使用泛型参数 T，该参数必须实现 Describable 和 Clone 两个 trait
fn some_function<T: Describable + Clone>(item: T) {
    let item_copy = item.clone(); // 克隆 item
    println!("Description: {}", item_copy.describe());
}

// 定义一个函数，返回一个实现了 Describable trait 的类型
fn return_description() -> impl Describable {
    Person {
        name: String::from("Lisa"),
        age: 18,
    }
}

// 定义一个继承了 Describable 的 trait Printable，并提供一个默认实现的方法 print
trait Printable: Describable {
    fn print(&self) {
        println!("{}", self.describe());
    }
}

// 为结构体 Person 实现 Printable trait
impl Printable for Person {}

fn main() {
    // 创建一个 Person 实例
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // 调用 describe 方法
    println!("{}", alice.describe());

    // 调用 default_description 方法
    println!("{}", alice.default_description());

    // 使用 output_description 函数
    output_description(&alice);
    // 使用 some_function 函数，传递一个克隆的 alice
    some_function(alice.clone());

    // 调用 return_description 函数并打印其描述
    println!("{}", return_description().describe());

    // 创建另一个 Person 实例，并调用 print 方法
    let bob = Person {
        name: String::from("Bob"),
        age: 25,
    };
    bob.print();
}

