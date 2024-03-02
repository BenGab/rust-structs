#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    let rect2 = Rectangle {
        height: 30,
        width: 40,
    };

    let rect3 = Rectangle {
        height: 20,
        width: 40,
    };

    let rect4 = Rectangle::square(10);

    dbg!(&rect1);
    println!("The are of rectangle is {} square pixels.", rect1.area());
    println!("The rect1 can hold rect2 {}", rect1.can_hold(&rect2));
    println!("The rect1 can hold rect3 {}", rect1.can_hold(&rect3));
    println!("The rect1 can hold rect4 {}", rect1.can_hold(&rect4));
}

