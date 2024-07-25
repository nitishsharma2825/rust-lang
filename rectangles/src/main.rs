#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32
}

fn main() {

    let rect1 = Rectangle {
        w: 30,
        h: 50
    };

    println!("The area of the rectange is {}.", area(&rect1));
    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
}

fn area(w: &Rectangle) -> u32 {
    w.w*w.h
}