use crate::zp;
use rug::Integer;

// large prime p, g in F_p, private key 1 <= a <= p - 1
pub fn generate_public_key(p: &Integer, g: &Integer, a: &Integer) -> Integer {
    zp::power(g, a, p)
}

pub fn encrypt(m: &Integer, k: &Integer, A: &Integer, p: &Integer, g: &Integer) -> (Integer, Integer) {
    let c1 = zp::power(g, k, p);
    let c2 = (zp::power(A, k, p) * m) % p;
    (c1, c2)
}

pub fn decrypt(c1: &Integer, c2: &Integer, a: &Integer, p: &Integer) -> Integer {
    (zp::inverse(&zp::power(c1, a, p), p) * c2) % p
}
