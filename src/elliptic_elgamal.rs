use rug::Integer;
use crate::elliptic_curves::*;

// prime p, elliptic curve E over Fp, point P in E(Fp)
#[derive(Debug)]
pub struct EllipticElgamal {
    pub p: Integer,
    pub E: EllipticCurve,
    pub P: Point
}

impl EllipticElgamal {
    pub fn generate_public_key(&self, nA: &Integer) -> Point {
        todo!()
    }

    pub fn encrypt(&self, M: &Point, k: &Integer) -> (Point, Point) {
        todo!()
    }

    pub fn decrypt(c1: &Point, c2: &Point, nA: &Integer) -> Point {
        todo!()
    }
}
