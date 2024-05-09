// Defining an Enum
enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    v4(String),
    v6(String),
}

enum IpAddr2 {
    v4(u8, u8, u8, u8),
    v6(String),
}

fn route(ib_kind: IpAddrKind) {}

// enum message
enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
let m = Message::Write(String::from("hello"));
m.call();

fn main() {
    // Enum Values
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    // Emum IpAddr
    let home_ip = IpAddr::v4(String::from("127.0.0.1"));
    let loopback_ip = IpAddr::v6(String::from("::1"));

    // Enum IpAddr2
    let home_ip2 = IpAddr2::v4(127, 0, 0, 1);
    let loopback_ip2 = IpAddr2::v6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

//-------- The match Control Flow Construct --------------
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// --- Patterns That Bind to Values
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
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// ---- Matching with Option <T> ----
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// --- Catch-all Patterns and the _ Placeholder ----
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat() {},
    7 => remove_fancy_hat() {},
    //other => move_player(other),
    //_ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
//fn move_player(num_spaces: u8) {}
//fn reroll() {}

// --- Concise Control Flow with if let
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maxinum is configured to be {}", max);
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}1", state);
} else {
    count += 1;
}
