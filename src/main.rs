fn main() {
    // Mutable references have one big restriction: you can have only
    // one mutable reference to a particular piece of data at a time. 
    let mut s1 = String::from("Hello");

    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);
    // r1, r2 are not used after this (their scope ends)

    // the mutable reference is now allowed. In the scope there are no
    // other immutable references
    let r3 = &mut s1;
    println!("{}", r3);

    // of course if this code was used, the scope of the immutabel r1, r2
    // would overlap with the scope of the mutable reference, so we would have
    // error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
    //
    //println!("{}, {}", r1, r2);
}
