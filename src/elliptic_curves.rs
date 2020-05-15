use rug::Integer;

// E: Y^2 = X^3 + AX + B
// A, B in Fp, p >= 3
#[derive(Debug)]
pub struct EllipticCurve {
    pub p: Integer,
    pub a: Integer,
    pub b: Integer,
}

#[derive(Debug)]
pub struct Point {
    pub x: Integer,
    pub y: Integer
}

// returns true if q is a quadratic residue mod p, false if otherwise
// uses Euler's criterion
pub fn is_quadratic_residue(q: &Integer, p: &Integer) -> bool {
    todo!()
}

impl EllipticCurve {
    // Checks if the point q is in this elliptic curve
    pub fn contains_point(&self, q: &Point) -> bool {
        todo!()
    }

    // some kind of function to return points where x coordinate is on the curve
    pub fn get_points(&self, x: &Integer) -> Vec<Point> {
        todo!()
    }
}

// returns the sum of two points a and b in elliptic curve E
pub fn add(a: &Point, b: &Point, E: &EllipticCurve) -> Point {
    todo!()
}

// returns the product of a point and a scalar in elliptic curve E
// use repeated addition, throw error if n <= 0
pub fn multiply(a: &Point, n: &Integer, E: &EllipticCurve) -> Point {
    todo!()
}
