fn main() {
    let w = 30;
    let h = 50;

    println!("The area of the rectange is {}.", area(w, h));
}

fn area(w: u32, h: u32) -> u32 {
    w*h
}