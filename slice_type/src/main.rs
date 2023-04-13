fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    let slice = &mut s[0..word];

    println!("word '{}'", slice);

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}