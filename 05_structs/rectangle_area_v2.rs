#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, comp_rectangle: &Rectangle) -> bool {
        self.width > comp_rectangle.width && self.height > comp_rectangle.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 59,
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "Rectangles areas rect1: {}, rec2: {}, rect3: {}",
        rectangle1.area(),
        rectangle2.area(),
        rectangle3.area()
    );

    println!(
        "Can rectangle1 hold rectangle2? {}",
        rectangle1.can_hold(&rectangle2)
    );

    println!(
        "Can rectangle1 hold rectangle3? {}",
        rectangle1.can_hold(&rectangle3)
    );

    println!("Square: {:?}", Rectangle::square(15));
}
