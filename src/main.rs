fn main() {
    let ref_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s
} // s goes out of scope and is freed. The reference returned is dangling
// ie, a pointer that references a location in memory that may have been given to someone else-