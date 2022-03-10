struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width 
        && self.height > other.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 50,
        height: 10,
    };

    let r2 = Rectangle {
        width: 60,
        height: 20,
    };

    let mut i = r1.can_hold(&r2);
    println!("{}", i);

    i = r2.can_hold(&r1);
    println!("{}", i);
}
