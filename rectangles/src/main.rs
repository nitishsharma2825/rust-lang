#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        w: 30,
        h: 50
    };

    let rect2 = Rectangle {
        w: 10,
        h: 40
    };

    let rect3 = Rectangle {
        w: 60,
        h: 45
    };

    let sq = Rectangle::square(3);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}