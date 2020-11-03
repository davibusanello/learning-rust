fn main() {
    another_function(31);
    new_function();
    let x = five();
    println!("Value of x: {}", x);
}

fn another_function(value: i32) {
    println!("The value is: {}", value);
}

fn new_function() {
    let _x = {
        let y = 6;
        y
    };
}
fn five() -> i32 {
    5 + 1
}
