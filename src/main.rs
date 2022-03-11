fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_hat(),
        4 => remove_hat(),
        // we have not listed all possible values. However, the last
        // arm is a catch-all
        foooo => move_player(), // book says `other`, but is `other` special
        5 => remove_hat(),
    }
}

fn add_hat() {}
fn remove_hat() {}
fn move_player() {}