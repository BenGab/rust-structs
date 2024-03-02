#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("The rect1 is {:?}", rect1);
    println!("The are of rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
