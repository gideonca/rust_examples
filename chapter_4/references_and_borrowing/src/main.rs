fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s2 = String::from("hello");
    println!("The value of s2 not changes is '{}'", s2);
    change(&mut s2);
    println!("The mutated value of s2 is '{}'", s2);

}

// fn takes a pointer to a String variable to keep from taking ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

// in order to change a variable, you have to pass a pointer to a mutuable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

