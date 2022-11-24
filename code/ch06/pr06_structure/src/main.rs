#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("Area of this rectangle '{:?}' is {}", rect1, rect1.area());
    println!("Perimeter of this rectangle '{:?}' is {}", rect1, rect1.perimeter());
}
