use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

fn risky_operation() -> Result<(), MyError> {
    if true { // 假定这里是一些业务逻辑的结果
        Err(MyError { message: String::from("Something went wrong") })
    } else {
        Ok(())
    }
}


fn divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn get_data_from_file() -> Result<String, std::io::Error> {
    let path = "data.txt";
    let content = std::fs::read_to_string(path)?;

    Ok(content)
}

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }


    match get_data_from_file() {
        Ok(data) => println!("File content: {}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }


    match risky_operation() {
        Ok(data) => println!("File content: {:?}", data),
        Err(e) => println!("Failed to read file: {}", e),
    }

    panic!("This is a critical error!");
}

