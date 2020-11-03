fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // print!("Index: {} \n", i);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut test_string = String::from("Hello world");
    let word = first_word(&test_string);
    test_string.clear();

    println!("test_string = {}", test_string);
    println!("word = {}", word);
}
