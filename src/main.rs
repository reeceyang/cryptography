mod zp;
mod elgamal;
mod primality;
mod encoding;
mod tests;
mod rsa;
mod elliptic_curves;
mod elliptic_elgamal;

use rug::Integer;
use std::env;
use std::fs;
use std::io;

fn main() {

    //tests::test_primality();
    //tests::test_rsa();

    let mut input = String::new();
    let mut system = String::new();
    let mut operation = String::new();

    print!("Choose (1-2):\n1. Encrypt 2. Decrypt\n> ");
    io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    match input.as_str().trim() {
        "1" => operation = "encrypt".to_string(),
        "2" => operation = "decrypt".to_string(),
        _ => panic!("Invalid input!")
    }

    print!("Choose (1-2):\n1. RSA 2. Elgamal\n> ");
    io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    match input.as_str().trim() {
        "1" => system = "rsa".to_string(),
        "2" => system = "elgamal".to_string(),
        _ => panic!("Invalid input!")
    }


}
