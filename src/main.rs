mod structs;
mod utils;
mod slices;
mod ownership;
mod enums;
mod common_collections;
mod error_handling;

use std::io::stdin;


fn main() {
    utils::print_header("start");
    println!("Enter sub-system:\n\
    \t1 -> ownership\n\
    \t2 -> slices\n\
    \t3 -> structs\n\
    \t4 -> enums\n\
    \t5 -> common_collections\n\
    \t6 -> errorHandling\n\
    \t -> Exit", );
    let mut input_string = String::new();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("No value... Exiting program");
    match input_string.trim() {
        "1" => { ownership::ownership(); }
        "2" => { slices::slices(); }
        "3" => { structs::structs(); }
        "4" => { enums::enums(); }
        "5" => { common_collections::collections(); }
        "6" => { error_handling::error_handling(); }
        _ => { utils::print_header("Invalid value: ... Exiting program") }
    }
    utils::print_header("End of program");
}









