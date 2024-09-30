// 定义一个泛型函数 display，接受一个实现了 std::fmt::Debug trait 的泛型参数，并打印它
fn display<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

// 定义一个泛型结构体 Point，包含两个字段 x 和 y
struct Point<T> {
    x: T,
    y: T,
}

// 为结构体 Point 实现关联函数和方法
impl<T> Point<T> {
    // 关联函数 new，创建一个新的 Point 实例
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // 方法 x，返回 Point 实例的 x 字段的引用
    fn x(&self) -> &T {
        &self.x
    }
}


// 定义一个泛型枚举 MyOption，用于表示一个可能包含值的类型
enum MyOption<T> {
    Some(T),
    None,
}

// 定义一个泛型结构体 Stack，包含一个 Vec<T> 用于存储栈中的元素
struct Stack<T> {
    items: Vec<T>,
}

// 为结构体 Stack 实现关联函数和方法
impl<T> Stack<T> {
    // 关联函数 new，创建一个新的空栈
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // 方法 push，向栈中压入一个元素
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // 方法 pop，从栈中弹出一个元素，返回一个 Option 类型
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// 定义一个泛型函数 largest，接受一个元素类型为 T 的切片，并返回切片中最大的元素
// 泛型参数 T 必须实现 PartialOrd 和 Clone trait
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    // 初始化变量 largest，存储切片中的第一个元素
    let mut largest = list[0].clone();

    // 遍历切片中的每个元素，如果元素大于 largest，则更新 largest
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }

    // 返回切片中的最大元素
    largest
}

fn main() {
    display(42); // 显示整数
    display("Hello, Rust!"); // 显示字符串

    // 创建两个 Point 实例，分别使用整数和浮点数作为泛型参数
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    // 打印 Point 实例的 x 和 y 字段
    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);

    // 创建一个 MyOption 枚举实例，包含一个整数值
    let some_number = MyOption::Some(42);
    // 创建一个 MyOption 枚举实例，不包含任何值
    let _no_number: MyOption<i32> = MyOption::None;

    // 使用 match 语句解构 MyOption 枚举实例
    match some_number {
        MyOption::Some(value) => println!("We have a number: {}", value),
        MyOption::None => println!("No number found"),
    }

    // 使用 Point 的 new 方法创建一个新的 Point 实例，并打印 x 字段
    let point = Point::new(5, 10);
    println!("Point x: {}", point.x());

    // 创建并使用泛型 Stack 结构体
    let mut stack_of_int = Stack::new();
    stack_of_int.push(1);
    stack_of_int.push(2);

    let mut stack_of_str = Stack::new();
    stack_of_str.push("Hello");
    stack_of_str.push("World");

    // 从栈中弹出元素并打印
    println!("{:?}", stack_of_int.pop());
    println!("{:?}", stack_of_str.pop());

    // 创建一个整数向量，并使用 largest 函数找到其中的最大值
    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&numbers));
}
