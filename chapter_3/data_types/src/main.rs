use std::io;

fn main() {
    
    // floating point numbers
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // numeric operations
    let _sum = 5 + 10; 
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // boolean type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // character type
    let _c = 'z';
    let _z: char = 'Z'; // with explicity type annotation
    let _heart_eyed_cat = '😻';

    // compound types
    // tuples
    let tup: (i32, f64, u8) = (500,6.4,1);
    let (_x, y, _z) = tup;
    println!("The value of y is {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");

    // array type
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5]; // declare data type of contents, and length
    let _c = [3; 5]; // declare data type of contents, and length

    let _first = a[0]; // access first element of array
    let _second = a[1]; // access second element of array

    // get user specified index of array, will throw IOB exception if value provided is outside of array
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
