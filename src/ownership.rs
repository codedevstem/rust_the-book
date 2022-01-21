#![allow(unused)]

use super::utils::wait;
pub fn ownership() {
    wait("ownership: move and copy");
    let s = String::from("Hello");
    takes_ownership(s); // <- 's' is moved into this function, no longer in scope
    let x = 5;
    makes_copy(x);
    println!("original: {}", x);

    wait("ownership: give, take and give back");
    let s1 = gives_ownership();
    let s2 = String::from("taken and given");
    let s3 = takes_and_gives_back(s2); // <- s2 is moved here and no longer in scope
    println!("given: {}", s1);
    // println!("{}", s2); // s2 compiler error if referenced
    println!("taken and given: {}", s3);

    wait("ownership: reference");
    let s4 = String::from("Hello World");

    let len = calculate_length(&s4);

    println!("Length of string: {}", len);

    wait("ownership: mutable reference");
    let mut s5 = String::from("Mutable Me");
    println!("Before: {}", s5);
    change(&mut s5);
    println!("After: {}", s5);
}

fn change(some_string: &mut String) {
    some_string.push_str(", injected");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn makes_copy(some_integer: i32) {
    println!("copy: {}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("ownership: {}", some_string);
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn gives_ownership() -> String {
    let some_string = String::from("given");
    some_string
}