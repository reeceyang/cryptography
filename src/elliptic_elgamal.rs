use rug::Integer;
use crate::elliptic_curves::*;

// prime p, elliptic curve E over Fp, point P in E(Fp)
#[derive(Debug)]
pub struct EllipticElgamal {
    pub E: EllipticCurve,
    pub P: Point
}

impl EllipticElgamal {
    pub fn generate_public_key(&self, nA: &Integer) -> Point {
        multiply(&self.P, nA, &self.E)
    }

    pub fn encrypt(&self, M: &Point, k: &Integer, Qa: &Point) -> (Point, Point) {
        let c1 = multiply(&self.P, k, &self.E);
        let c2 = add(&multiply(Qa, k, &self.E), M, &self.E);
        (c1, c2)
    }

    pub fn decrypt(&self, c1: &Point, c2: &Point, nA: &Integer) -> Point {
        add(c2, &multiply(c1, nA, &self.E).reflection(), &self.E)
    }
}
