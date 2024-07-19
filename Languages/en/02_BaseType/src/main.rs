fn main() {
let x_immutable = 5;
println!("The value of x_immutable is: {}", x_immutable);
// x_immutable = 6; // This line will cause a compilation error because x_immutable is immutable

let mut y_mutable = 5;
println!("The value of y_mutable is: {}", y_mutable);
y_mutable = 6; // This is allowed because y_mutable is mutable
println!("The value of y_mutable is: {}", y_mutable);

let x_int_neg: i32 = -123;
let y_uint_pos: u32 = 123;
println!("x_int_neg is {}, y_uint_pos is {}", x_int_neg, y_uint_pos);

let x_float64 = 2.0; // defaults to f64
let y_float32: f32 = 3.0; // explicitly declared as f32
println!("x_float64 is {}, y_float32 is {}", x_float64, y_float32);

let t_bool = true;
let f_bool: bool = false; // explicit type declaration
println!("t_bool is {}, f_bool is {}", t_bool, f_bool);

let c_char = 'z';
let z_char = 'ℤ';
let heart_eyed_cat_char = '😻';
let z_string = String::from("string");
println!("c_char is {}, z_char is {}, heart_eyed_cat_char is {}", c_char, z_char, heart_eyed_cat_char);
println!("The character 'c_char' occupies {} bytes of memory size", std::mem::size_of_val(&c_char));
println!("The string 'z_string' content occupies {} bytes of memory size", &z_string.as_bytes().len());

let tup_var: (i32, f64, u8, char) = (-500, 6.4, 1, 'z');
let (_w_from_tup, x_from_tup, _y_from_tup, _z_from_tup) = tup_var; // Deconstruct tuple
println!("The value of x_from_tup is: {}", x_from_tup);

let a_array = [1, 2, 3, 4, 5];
let first_element = a_array[0]; let second_element = a_array[1]; println!("The first element is {}, the second element is {}", first_element, second_element); }
