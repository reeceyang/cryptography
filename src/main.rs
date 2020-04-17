mod zp;
mod elgamal;
mod primality;
mod encoding;
mod tests;

use rug::Integer;
use std::env;
use std::fs;
use std::io;

fn main() {

    tests::test_primality();

    /*let mut input = String::new();
    let mut system = String::new();
    let mut operation = String::new();

    print!("Choose (1-2):\n1. Encrypt 2. Decrypt\n> ");
    io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    match input {
        &String::from("1") => operation = "encrypt".to_string(),
        &String::from("1") => operation = "decrypt"
    }*/

}
