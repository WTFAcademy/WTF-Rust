fn display(s: String) {
    println!("{:?}", s);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to s1
    s.len()
}

fn change(some_string: &mut String) -> &mut String {
    some_string.push_str(", wtf!");
    some_string
}

fn main() {
    let mut s1 = String::from("hello"); // s1 is the owner of the "hello" object
    s1.push_str(", wtf!"); // s1 can modify the "hello" object by appending a string

    let s2 = s1; // Ownership is transferred from s1 to s2
    // println!("{s1}"); // Error: s1 no longer owns the string

    display(s2); // s2 transfers ownership to the function parameter s; s2 becomes unusable
    // println!("{s2}"); // Error: s2 is no longer usable

    // Immutable Borrowing
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // s3 is immutably borrowed; the function can only read but not modify s3
    println!("The length of '{}' is {}.", s3, len);

    // Mutable Borrowing
    let mut s4 = String::from("hello");
    change(&mut s4); // s4 is mutably borrowed; the function can modify s4
    println!("{}", s4);
}
