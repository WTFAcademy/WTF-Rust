trait Describable {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}


fn output_description(item: impl Describable) {
    println!("{}", item.describe());
}

fn some_function<T: Describable + Clone>(item: T) {
    let item_copy = item.clone();
    println!("Description: {}", item.describe());
}


// 默认方法和继承 Trait
trait Printable: Describable {
    fn print(&self) {
        println!("{}", self.describe());
    }
}

impl Printable for Person {}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("{}", alice.describe());

    output_description(alice);
}

