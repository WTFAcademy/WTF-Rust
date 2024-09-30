fn print_hello() {
    println!("Hello, world!");
}

fn print_sum(a: i32, b: i32) {
    println!("Sum is: {}", a + b);
}

fn add_two(a: i32) -> i32 {
    a + 2
    // 或 `return a + 2;`
}

fn main() {
    print_hello();
    print_sum(11, 22);
    let add_two_result = add_two(33);
    println!("Add two result is {}", add_two_result);

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    loop {
        println!("again!");
        break; // 无限循环，但这里我们立刻跳出循环
    }


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..10 {
        if number % 2 == 0 {
            continue;
        }
        println!("Found an odd number: {}", number);
        if number == 7 {
            break;
        }
    }
}
