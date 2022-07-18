use std::default;

enum IPAddrKind {
    V4(String), //add data in parantheses to show the type of data u wanna store
    V6(String),
}

impl IPAddrKind {
    fn hello() {
        println!("Hello");
    }
}
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

pub fn run() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    let localhost = IPAddrKind::V4(String::from("127.0.0.1"));
    IPAddrKind::hello();

    //rust doesnt have null value
    /*enum Option<T> {
        Some(T),
        None,
    }*/

    //included in program scope by default

    let some_string = Some("hello");
    let absent: Option<i32> = None;

    //for a none value u have to specify types

    let x = Option::Some(5);
    let y = 12;

    // let sum = x + y; //cant perform operations optional types to normal types
    let sum = x.unwrap_or(0) + y;

    value(Coin::Quarter(USState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //match statements are exhaustive

    //if-let statements basically shorthand match
    let some_value = Some(3);

    if let Some(3) = some_value {
        println!("3 h");
    }
}

fn route(ip: IPAddrKind) {}

fn value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("pennywise");
            1
        }
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("state {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
