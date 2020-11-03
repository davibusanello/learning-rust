fn main() {
    let test_string = String::from("hello world");
    // let test_literal_string = "hello world";

    // Use slices of String
    let first = first_word(&test_string[..]);

    // Use slices of literal string
    // let first = first_word(&test_literal_string[..]);

    // Use the literal string
    // let first = first_word(test_literal_string);
    let second = second_word(&test_string);

    println!("The first word: {}", first);
    println!("The second word: {}", second);
}

fn first_word(full_string: &str) -> &str {
    let bytes = full_string.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &full_string[..index];
        }
    }

    &full_string[..]
}

fn second_word(full_string: &String) -> &str {
    let bytes = full_string.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &full_string[index+1..]
        }
    }

    &full_string[..]
}
