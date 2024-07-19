use itertools::Itertools;

fn main() {
    let mixed = ("Rust", 2023, 3.14, true);
    let (lang, year, pi, status) = mixed;

    println!("Language: {}, Year: {}, PI: {}, Status: {}", lang, year, pi, status);


    // 1. Explicitly declare the type of the dynamic array
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("vector v1: {:?}", &v1);

    // 2. Use the macro vec! to create an array with initial values
    let v2 = vec![1u8, 2, 3];  
    println!("vector v2: {:?}", &v2);

    let mut s = String::from("Hello"); // Mutable String type
    s.push_str(", world!"); // Modify the String
    println!("{}", s);

    let slice = &s[0..5]; // Get a slice of the String; the first parameter is the start index (inclusive), the second parameter is the end index (exclusive)
    println!("Slice: {}", slice); // Hello


    let numbers = [1, 2, 3, 4, 5];
    // Add numbers 6 and 7 to the array numbers

    let slice = &numbers[0..2]; // Reference a part of the array

    for &item in slice.iter() {
        println!("{}", item);
    }
    slice.iter().for_each(|&item| println!("{}", item));
}
