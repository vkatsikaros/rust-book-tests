#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime    => {
            dbg!(&coin);
            10
        },
        Coin::Quarter => 25,
    }
}

fn main() {
    let x = Coin::Dime;
    let cents = value_in_cents(x);
    println!("{}", cents);
}
