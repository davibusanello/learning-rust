fn main () {
    let phrase = String::from("hello world");

    let word = first_word(&phrase[..]);

    println!("First word: {}", word);

    let literal_string_phrase = "hello world";
    let word = first_word(&literal_string_phrase[..]);
    println!("second First word: {}", word);
    let word = first_word(literal_string_phrase);

    println!("third First word: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
