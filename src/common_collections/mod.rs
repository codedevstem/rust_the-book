#![allow(unused)]
use super::utils::wait;
use std::collections::HashMap;
pub fn collections() {
    let v: Vec<i32> = Vec::new();
    let _v = vec![1,2,3];

    let mut mut_vector = Vec::new();
    mut_vector.push(5);
    mut_vector.push(6);
    mut_vector.push(7);
    mut_vector.push(8);
    wait("common_collections: vector element (matching?)");
    let third: &i32 = &mut_vector[2];
    println!("The third element is {}", third);

    match mut_vector.get(2) {
        Some(third) => {println!("The third element is {}", third);}
        None => {println!("There is no third element.");}
    }

    let iter_vector = vec![100, 32, 57];
    println!("Iterating over vector: {:?}", iter_vector);
    for i in &iter_vector {
        println!("{}", i);
    }
    wait("common_collections: string updating");
    let mut s = String::from("foo");
    println!("push_str before: {}", s);
    s.push_str("bar");
    println!("push_str before: {}", s);
    wait("common_collections: HashMaps");
    let mut manual_scores = HashMap::new();
    manual_scores.insert(String::from("blue"), 10);
    manual_scores.insert(String::from("yellow"), 50);

    println!("manual hashmap: {:?}", manual_scores);

    let teams = vec![String::from("blue"),String::from("yellow")];
    let initial_scores = vec![10, 50];
    let mut automatic_scores: HashMap<_,_> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("zipped hashmap: {:?}", automatic_scores);

}