fn main() {
    // Mutable references have one big restriction: you can have only
    // one mutable reference to a particular piece of data at a time. 
    let mut s1 = String::from("Hello");

    let r1 = &s1;
    let r2 = &s1;
    let r3 = &mut s1;

    println!("{}, {}, {}", r1, r2, r3);
}
