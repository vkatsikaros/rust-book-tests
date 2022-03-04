fn main() {
    let s1 = String::from("Hello");

    // refer to value s1 without taking ownership
    let len = calculate_length(&s1);
    println!("The lenght of '{}' is '{}'", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String (its "structure" not the actual value)
    s.len()
    // s goes out of scope. It's a reference so it doesn't have ownership of
    // the value, and nothing happens
}
