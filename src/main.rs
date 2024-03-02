#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    dbg!(&rect1);
    println!("The are of rectangle is {} square pixels.", rect1.area());
}

