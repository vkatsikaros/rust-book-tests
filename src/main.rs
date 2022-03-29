fn main() {
    let mut v = vec![1,2,3,4,5];

    let f = &v[0];
    v.push(6);
    println!("{}", f);
}