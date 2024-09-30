fn main() {
    let x_immutable = 5;
    println!("The value of x_immutable is: {}", x_immutable);
    // x_immutable = 6; // 这行会导致编译错误，因为 x_immutable 不可变

    let mut y_mutable = 5;
    println!("The value of y_mutable is: {}", y_mutable);
    y_mutable = 6; // 这是允许的，因为 y_mutable 是可变的
    println!("The value of y_mutable is: {}", y_mutable);

    let x_int_neg: i32 = -123;
    let y_uint_pos: u32 = 123;
    println!("x_int_neg is {}, y_uint_pos is {}", x_int_neg, y_uint_pos);

    let x_float64 = 2.0; // 默认为 f64
    let y_float32: f32 = 3.0; // 显式声明为 f32
    println!("x_float64 is {}, y_float32 is {}", x_float64, y_float32);

    let t_bool = true;
    let f_bool: bool = false; // 显式类型声明
    println!("t_bool is {}, f_bool is {}", t_bool, f_bool);

    let c_char = 'z';
    let z_char = 'ℤ';
    let heart_eyed_cat_char = '😻';
    let y_char = '棒';
    let z_string = String::from("棒");
    println!("c_char is {}, z_char is {}, heart_eyed_cat_char is {}, y_char is {}", c_char, z_char, heart_eyed_cat_char, y_char);
    println!("字符'c_char'占用了{}字节的内存大小", size_of_val(&c_char));
    println!("字符'y_char'占用了{}字节的内存大小", size_of_val(&y_char));
    println!("字符串'z_string'内容占用了{}字节的内存大小", &z_string.as_bytes().len());

    let tup_var: (i32, f64, u8, char) = (-500, 6.4, 1, 'z');
    let (_w_from_tup, x_from_tup, _y_from_tup, _z_from_tup) = tup_var; // 解构元组
    println!("The value of x_from_tup is: {}", x_from_tup);

    let a_array = [1, 2, 3, 4, 5];
    let first_element = a_array[0];
    let second_element = a_array[1];
    println!("The first element is {}, the second element is {}", first_element, second_element);
}
