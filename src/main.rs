#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let i = 3;
    let r1 = Rectangle {
        width: dbg!(50 * i),
        height: 10,
    };

    let area = area(&r1);
    dbg!(&r1);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}