use rug::Integer;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use crate::primality;
use crate::encoding;
use crate::elgamal;

pub fn test_primality() -> Result<(), Error> {
    let path = "primes.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut primes = 0;

    for line in buffered.lines() {
        let copy = line?.clone();
        let isprime = primality::is_prime(&(copy.parse::<Integer>().unwrap()));
        if isprime {
            primes += 1;
        }
        println!("is {0} prime? {1}", copy, isprime);
    }

    println!("found {} primes", primes);

    Ok(())
}

pub fn test_encoding() {
    // Get file name from commandline
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    // Read it the content of the file, store the byte codes in array "bytes"
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let bytes = contents.as_bytes();
    let bytes = bytes.iter().map(|x| Integer::from(*x as i8)).collect();

    //let v: Vec<Integer> = vec![Integer::from(72), Integer::from(101), Integer::from(108), Integer::from(108), Integer::from(111), Integer::from(32), Integer::from(119), Integer::from(111), Integer::from(114), Integer::from(108), Integer::from(100), Integer::from(33), Integer::from(32), Integer::from(10), Integer::from(10), Integer::from(10), Integer::from(10), Integer::from(10)];
    //println!("{}", encoding::to_decimal(&String::from("1101")));
    //println!("{}", encoding::to_binary_string(&Integer::from(14)));

    // Comments from Dr. Weiss
    // bytes are 8 bits, so one blockSize is 8. If the other blockSize is a multiple of 8,
    // you just have to kill the redundant zeros that may occur. This is easy for a text,
    // but iffy for a binary file which may have legitimate zeros.
    // A solution to this would be to transmit metadata about the blocksize of each block,
    // in order to fill in the appropriate number of leading zeros.
    println!("{:?}", bytes);
    let old = encoding::block_convert(&bytes, 8, 32);
    println!("{:?}", old);
    let new = encoding::block_convert(&old, 32, 8);
    println!("{:?}", new);
}

fn test_elgamal() {
    let p = "829117788519050559035918710717997636329955988917709922673600843823699223723218893772406842608793835590020958924831614596288658153095200390460598343396417689857928142523613363841665360379637639598934024341681447625443250160875687874241390985874834880987594594654481984890308637067120470716212207494729".parse::<Integer>().unwrap();
    let g = "".parse::<Integer>().unwrap();
    let a = "".parse::<Integer>().unwrap();
    let mine = elgamal::generate_public_key(&p, &g, &a);
    println!("A = {}", mine);

    let c1 = "".parse::<Integer>().unwrap();
    let c2 = "".parse::<Integer>().unwrap();

    let m = elgamal::decrypt(&c1, &c2, &a, &p);
    println!("{}", m);

    /*
    let m = Integer::from(42069i64);
    let k = Integer::from(3);
    let (c1, c2) = elgamal::encrypt(&m, &k, &jiaua, &p, &g);
    println!("c1 = {}", c1);
    println!("c2 = {}", c2);
    */
}
