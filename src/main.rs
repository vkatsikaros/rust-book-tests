#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle::square(10);
    dbg!(r1);
}
