fn display<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}


struct Point<T> {
    x: T,
    y: T,
}


struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}

fn main() {
    display(42); // 显示整数
    display("Hello, Rust!"); // 显示字符串


    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);


    let mut stack_of_int = Stack::new();
    stack_of_int.push(1);
    stack_of_int.push(2);

    let mut stack_of_str = Stack::new();
    stack_of_str.push("Hello");
    stack_of_str.push("World");

    println!("{:?}", stack_of_int.pop());
    println!("{:?}", stack_of_str.pop());


    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&numbers));
}