fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    loop {
        println!("The value of number is: {}", number);
    }
}
