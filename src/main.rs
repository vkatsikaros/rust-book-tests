//
// https://doc.rust-lang.org/reference/patterns.html#patterns
//
// Patterns are used to match values against structures 
// and to, optionally, bind variables to values inside 
// these structures. They are also used in variable 
// declarations and parameters for functions and closures.

fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_hat(),
        4 => remove_hat(),
        // Wildcard pattern
        // https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern
        _ => move_player(),
    }
}

fn add_hat() {}
fn remove_hat() {}
fn move_player() {}