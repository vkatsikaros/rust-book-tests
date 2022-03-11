#[derive(Debug)]
enum Fruit {
    Apple,
    Banana,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime(Fruit),
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny   => 1,
        Coin::Nickel  => 5,
        Coin::Dime(state)    => {
            println!("Dime with {:?}", state);
            10
        },
        Coin::Quarter => 25,
    }
}

fn main() {
    let x = Coin::Dime(Fruit::Apple);
    let cents = value_in_cents(x);
    println!("{}", cents);
}
