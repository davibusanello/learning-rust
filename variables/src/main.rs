fn main() {
    let guess: f32 = 15000.00;
    println!("Numero {}", guess);

    let char_type = '🍕';
    println!("Char type {}", char_type);

    let tup: (i32, f64, u8) = (400, 10.666, 1);
    // let (x, y, z) = tup;
    println!("Tuple: {}", tup.2);

    let array_type: [i32; 5] = [1,2,3,4,5];

    let index = 15;

    let element = array_type[index];

    println!("The value is: {}", element);
}
