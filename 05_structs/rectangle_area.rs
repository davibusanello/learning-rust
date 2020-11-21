#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rectangle = Rectangle {
        width: width1,
        height: height1,
    };
    println!("rect {:#?}", rectangle);

    println!(
        "Common: The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "Tuples: The area of the rectangle is {} square pixels.",
        area_with_tuple((width1, height1))
    );

    println!(
        "Structs: The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
