struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 50,
        height: 10,
    };

    let area = r1.area();
    println!("{}", area);
}
