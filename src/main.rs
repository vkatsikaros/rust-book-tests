fn main() {
    let s1 = String::new();
    println!("{}", s1);

    let s2 = String::from("helloooooo");
    println!("{}", s2);

    let data = "hello!";
    let s3 = data.to_string();
    println!("{}", s3);

    let s4 = "hello???".to_string();
    println!("{}", s4);

    let ar = String::from("السلام عليكم");
    println!("{}", ar);

    let gr = String::from("Καλημέεεεεεερα");
    println!("{}", gr);

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{}", s5);

    let s6 = String::from("foo");
    let s7 = String::from("bar");
    let space = String::from(" ");
    let fb = s6 + &space + &s7;
    // + (or `add`) takes ownership of the take ownership of 1st argument, s6
    // the rest of the arguments are references, whose contents are copied, space and s7
    // and then returns ownership of the results
    // So... s6 is not valid afterwards
    println!("{}", fb);

    let s8 = format!("{}-{}-{}-{}-{}", ar, s2, s3, s4, s5);
    println!("{}", s8);

    // hindi word: नमस्ते
    // bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // grapheme clusters/letters ['न', 'म', 'स', '्', 'त', 'े']
    let hi = String::from("नमस्ते");
    println!("{}", hi);

    // https://doc.rust-lang.org/std/string/struct.String.html#method.chars
    println!("Print characters");
    for c in hi.chars() {
        println!("{}", c);
    }

    // https://doc.rust-lang.org/std/string/struct.String.html#method.bytes
    println!("Print bytes");
    for c in hi.bytes() {
        println!("{}", c);
    }
}
