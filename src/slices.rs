#![allow(unused)]

use super::utils::wait;

pub fn slices() {
    wait("slices: first name method");
    let mut sentence = "Kristian Hamre-Os";
    let word = first_word(&sentence);
    println!("first word is: {}", word);

    wait("slices: array");
    let a = [1,2,3,4,5];
    let b = &a[1..3];
    println!("original slice: {:?}", a);
    println!("index [1..3]: {:?}", b);
    println!("this makes slices in rust inclusive..exclusive")
}

pub fn first_word(input: &str) -> &str {
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &input[..i]
        }
    }
    &input[..]
}