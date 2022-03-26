fn main() {
    let v = vec![1,2,3,4,5];

    let t: &i32 = &v[2];
    println!("{}", t);

    match v.get(2) {
        // need to come back and understand why we can put a variable in
        // the Some() that is not the match expression
        Some(t) => println!("3rd is: {}", t),
        None => println!("no 3rd element"),
    }
}