
fn main() {
    let v = vec![1,2,3,4,5];

    // reading from out of bounds will crash the program
    //let x = &v[100];

    // reading from out of bounds returns None
    let x = v.get(100);
    dbg!(x);

}