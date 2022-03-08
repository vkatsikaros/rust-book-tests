struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let r1 = Rectangle {
        width: 50,
        height: 10,
    };

    let area = area(r1);
    println!("Area {}", area);
}

fn area(r: Rectangle) -> u32 {
    r.width * r.height
}