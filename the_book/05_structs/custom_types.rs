fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black color values {}, {}, {}", black.0, black.1, black.2);
}


// Unit-like Struct
//  ()
