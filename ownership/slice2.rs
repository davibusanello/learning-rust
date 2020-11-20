fn main () {
    let mut phrase = String::from("hello world");

    let word = first_word(&phrase);
    phrase.clear();

    println!("First word: {}", word);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
