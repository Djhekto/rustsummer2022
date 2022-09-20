#![allow(unused)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let xxx = Coin::Penny;
    value_in_cents(xxx);
    let xx:Coin;
    xx = Coin::Penny;
    value_in_cents(xx);
}