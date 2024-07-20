// Define a Person struct
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Define a method to display personal information
    fn show_info(&self) {
        println!("{} is {} years old.", self.name, self.age);
    }
}

fn show_info() {
    println!("this is a function");
}

fn main() {
    // Create a Person instance with the name Alice and age 30
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Call the method to display personal information
    person1.show_info();

    // Call this function without needing an owner
    show_info();
}
