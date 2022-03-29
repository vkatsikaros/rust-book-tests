fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v2 = vec![6, 7, 8, 9, 10];

    // loop over immutable references
    for i in &v1 {
        println!("{}", i);
    }

    // loop over mutable references
    for i in &mut v2 {
        *i += 10;
        println!("{}", i);
    }
}
