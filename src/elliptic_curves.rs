use rug::Integer;
use crate::zp;

#[derive(Debug)]
pub struct Point {
    pub x: Integer,
    pub y: Integer
}

impl Point {
    pub fn reflection(&self) -> Point {
        Point {x: self.x.clone(), y: -self.y.clone()}
    }
}

// E: Y^2 = X^3 + AX + B
// A, B in Fp, p >= 3
#[derive(Debug)]
pub struct EllipticCurve {
    pub p: Integer,
    pub a: Integer,
    pub b: Integer,
}

// returns true if q is a quadratic residue mod p, false if otherwise
// uses Euler's criterion
pub fn is_quadratic_residue(q: &Integer, p: &Integer) -> bool {
    todo!()
}

impl EllipticCurve {
    // Checks if the point q is in this elliptic curve
    pub fn contains_point(&self, q: &Point) -> bool {
        // would rather not use .clones
        // println!("{}", (q.x.clone() * q.x.clone() * q.x.clone() + self.a.clone() * q.x.clone() + self.b.clone() - q.y.clone() * q.y.clone()) % self.p.clone());
        if (q.x.clone() * q.x.clone() * q.x.clone() + self.a.clone() * q.x.clone() + self.b.clone() - q.y.clone() * q.y.clone()) % self.p.clone() == 0 {
            return true;
        }
        false
    }

    // some kind of function to return points where x coordinate is on the curve
    pub fn get_points(&self, x: &Integer) -> Vec<Point> {
        todo!()
    }
}

// returns the sum of two points a and b in elliptic curve E
pub fn add(a: &Point, b: &Point, E: &EllipticCurve) -> Point {
    // if a = 0, then a + b = b
    // if b = 0, then a + b = a
    if a.x == b.x && a.y == Integer::from(-&b.y) {
        panic!("Point a and point b add to O");
    }
    let mut lambda;
    if a.x == b.x && a.y == b.y {
        let prod = Integer::from(&a.y * 2);
        //println!("prod = {}", prod);
        let denom = zp::inverse(&Integer::from(&prod % &E.p), &E.p);
        //println!("denom = {}", denom);
        lambda = Integer::from(denom * (Integer::from(3) * &a.x * &a.x + &E.a));
    } else {
        let diff = Integer::from(&b.x - &a.x) + &E.p;
        let denom = zp::inverse(&Integer::from(&diff % &E.p), &E.p);
        lambda = denom * Integer::from(&b.y - &a.y);
    }
    lambda = lambda % &E.p;
    //println!("lambda = {}", lambda);
    let mut x3 = &lambda * &lambda - Integer::from(&a.x + &b.x);
    while x3 < 0 {
        x3 = Integer::from(&x3 + &E.p);
    }
    let mut y3 = &lambda * Integer::from(&a.x - &x3) - &a.y;
    while y3 < 0 {
        y3 = Integer::from(&y3 + &E.p);
    }
    Point {x: x3 % &E.p, y: y3 % &E.p}
}

// returns the product of a point and a scalar in elliptic curve E
// use repeated addition, throw error if n <= 0
pub fn multiply(a: &Point, n: &Integer, E: &EllipticCurve) -> Point {
    if n <= &Integer::from(0) {
        panic!("n <= 0, aborted multiplication");
    }
    let mut prod = Point {x: a.x.clone(), y: a.y.clone()};
    let mut ind = Integer::from(1);
    while ind <= Integer::from(n - 1) {
        prod = add(&prod, &prod, E);
        ind = ind + 1;
    }
    prod
}
