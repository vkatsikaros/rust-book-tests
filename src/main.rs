fn main() {
    let mut s1 = String::from("Hello");

    change(&mut s1);
}

fn change(s: &mut String) {
    // the function can mutate the value it borrows

    // references are immutable, so this will throw an error
    s.push_str(", world");
}
