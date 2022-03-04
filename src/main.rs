fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x; // copy the value of x

    let s1 = String::from("Hello");
    let s2 = s1; // this is NOT copying the value of s1
    // instead the string "structure" (pointer, length, capacity)
    // of s1 is copied to s2, not the value
}
