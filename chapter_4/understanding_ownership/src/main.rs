fn main() {
    
    /*
    Ownership rules:
    1. Each value in Rust has an owner
    2. There can only be one owner at a time
    3. When the owner goes out of scope, the value will be dropped
    */
    
    {                                   // s is not valid here, it's not yet declared
        let s = String::from("hello");  // create a string from a litteral
                                        // using :: in this case applys from to the String namespace
                                        // s is valid from this point forward
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
    }                                   // this scope is now over, so s is no longer valid

    /*
    String has 3 parts:
    1. Pointer to memory that holds the content
    2. Length (number of bytes the content is currently using)
    3. Capacity (total amount of bytes that the string has received from the allocator)
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();             // data is copied from s1 to s2, both point to the same memory location

    println!("s1 = {}, s2 = {}", s1, s2);

    let _s3 = gives_ownership();         // gives_ownershihp moves its return value into s3
    let s4 = String::from("hello");     // s4 comes into scope
    let s5 = takes_and_gives_back(s4);  // s4 is moved into takes_and_gives_back
                                        // takes_and_gives_back, which also
                                        // moves its return value into s5

    println!("s5 {}", s5);

    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);
    println!("The length of '{}' is {}.", s7, len);

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

    some_string                          // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}