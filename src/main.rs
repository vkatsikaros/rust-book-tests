fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_give_back_ownership(s2);
}

fn gives_ownership() -> String {
    let something = String::from("Hello");
    something
    // something is returned and moves to the caller
}

fn takes_and_give_back_ownership(something: String) -> String {
    // something comes into scope
    something
    // something is returned and moves to the caller
}
