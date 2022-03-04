fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x; // copy the value of x

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // deep copy, the value of s1 is copied to s2

    println!("s1 = {}, s2 = {}", s1, s2);
}
