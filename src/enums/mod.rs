#![allow(unused)]
use super::utils::wait;

enum IpAddrKind {
    V4,
    V6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
/**
IpAddrString - This is documentation
*/
#[derive(Debug)]
enum IpAddrString {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) =>{
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

pub fn enums() {
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let home_concise = IpAddrString::V4(String::from("127.0.0.1"));
    let loopback_concise = IpAddrString::V6(String::from("::1"));

    println!("Home Address: {:?}", home_concise);
    println!("Loopback Address: {:?}", loopback_concise);

    wait("Enums: Option<T>");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    wait("Enums: Pattern matching");
    println!("Cent value of a penny: {}", Coin::Penny.value_in_cents());
    println!("Cent value of a nickel: {}", Coin::Nickel.value_in_cents());
    println!("Cent value of a dime: {}", Coin::Dime.value_in_cents());
    println!("Cent value of a quarter: {}", Coin::Quarter(UsState::Alaska).value_in_cents());
    println!("Cent value of a quarter: {}", Coin::Quarter(UsState::Alabama).value_in_cents());

    wait("Enums: Matching with Option<T>");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five);
    dbg!(six);
    dbg!(none);
}



