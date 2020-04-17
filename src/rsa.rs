use crate::primality::generate_random_prime;
use rug::Integer;
use crate::zp;
use crate::primality;

pub fn generate_random_p_q() -> (Integer, Integer) {
    (primality::generate_random_prime(&(32 as usize)), primality::generate_random_prime(&(32 as usize)))
}

pub fn generate_public_key(p: &Integer, q: &Integer) ->  (Integer, Integer) {
    let one = Integer::from(1);
    let N = Integer::from(p * q);
    let mut e = generate_random_prime(&(32 as usize));
    while zp::gcd(&e, &Integer::from((p - one.clone()) * (q - one.clone()))) != 1 {
        e = primality::generate_random_prime(&(32 as usize));
    }
    (N, e)
}

pub fn encrypt(m: &Integer, N: &Integer, e: &Integer) -> Integer {
    zp::power(&m, &e, &N)
}

pub fn decrypt(c: &Integer, e: &Integer, p: &Integer, q: &Integer, N: &Integer) -> Integer {
    let one = Integer::from(1);
    let d = zp::inverse(&e, &Integer::from((p - one.clone()) * (q - one.clone())));
    zp::power(&c, &d, &N)
}
