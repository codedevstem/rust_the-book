#![allow(unused)]
use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};
use super::utils::wait;

const USERNAME_FILEPATH: &str = "src/error_handling/username.txt";
pub fn error_handling() {
    let filepath = "src/error_handling/tmp.txt";
    wait("error handling method one");
    let f = File::open(filepath);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound =>  match File::create(filepath) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
    wait("error handling method two");
    let file = File::open(filepath).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filepath).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error)
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
    wait("Propagating Errors");

    let username = read_username_from_file().unwrap_or_else(|error| {
        panic!("failed to read username from file: {:?}", error)
    });

    println!("username from file: {}", username)

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open(USERNAME_FILEPATH);

    let mut f  = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    let mut f = File::open(USERNAME_FILEPATH)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorthander() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(USERNAME_FILEPATH)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorthandest() -> Result<String, io::Error> {
    fs::read_to_string(USERNAME_FILEPATH)
}