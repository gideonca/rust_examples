fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5);
    print_labeled_measurement(5,'h');

    // expressions
    let x = {
        let y = 3;
        y + 1;
    };

    println!("The valueof x is {x}");

    let x = plus_one(5);
    println!("The value of x is {x}");

}

fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value for x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}