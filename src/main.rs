fn main() {
    let s1 = String::from("Hello");

    takes_ownership(s1);
    // s1 value moved in function, so no longer valid

    let x = 5;

    makes_copy(x);
    // i32 implements Copy. x value is copied. So x is still valid
}

fn takes_ownership(something: String) {
    println!("{}", something);
}

fn makes_copy(something: i32) {
    println!("{}", something);
}
