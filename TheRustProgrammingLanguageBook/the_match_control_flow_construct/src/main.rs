fn main() {
    println!("Hello, world!");
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //Coin::Penny => 1,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        //Coin::Quarter => 25
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25,
        },
    }
}
