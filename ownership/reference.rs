fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(" ss");

    s.len()
}
