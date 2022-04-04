fn main() {
    let s1 = String::new();
    println!("{}", s1);

    let s2 = String::from("helloooooo");
    println!("{}", s2);

    let data = "hello!";
    let s3 = data.to_string();
    println!("{}", s3);

    let s4 = "hello???".to_string();
    println!("{}", s4);

    let ar = String::from("السلام عليكم");
    println!("{}", ar);

    let gr = String::from("Καλημέεεεεεερα");
    println!("{}", gr);

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{}", s5);
}
