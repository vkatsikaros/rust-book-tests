fn main() {
    // Mutable references have one big restriction: you can have only
    // one mutable reference to a particular piece of data at a time. 
    let mut s1 = String::from("Hello");

    // create one mutable reference
    let r1 = &mut s1;

    println!("{}", r1);
}
