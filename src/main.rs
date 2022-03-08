fn main() {
    let height = 50;
    let width = 10;

    let area = area(width, height);
    println!("Area {}", area);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}