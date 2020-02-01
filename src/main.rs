use rug::Integer;
use std::cmp;

#[derive(Debug)]
struct Zp {
    p: Integer,
    value: Integer
}

// this can probably be cleaned up - do we actually need all the clones?
impl Zp {
    fn add(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            // throw an error?
        }
        Zp {p: self.p.clone(), value: (self.value.clone() + a.value.clone()) % self.p.clone()}
    }
    fn subtract(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            // throw an error?
        }
        Zp {p: self.p.clone(), value: (self.value.clone() - a.value.clone() + self.p.clone()) % self.p.clone()}
    }
    fn multiply(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            // throw an error?
        }
        Zp {p: self.p.clone(), value: (self.value.clone() * a.value.clone()) % self.p.clone()}
    }

    // finds the inverse using the rug crate's built in inverse function
    fn native_inverse(&self) -> Zp {
        let inv = match self.value.clone().invert(&self.p) {
            Ok(inverse) => inverse,
            Err(_) => unreachable!(),
        };
        Zp {p: self.p.clone(), value: inv}
    }
}

// returns the gcd of two Integers using the Euclidean Algorithm
fn gcd(a: &Integer, b: &Integer) -> Integer {
    let mut r0: Integer = if a > b {a.clone()} else {b.clone()};
    let mut r1: Integer = if a > b {b.clone()} else {a.clone()};
    loop {
        let r: Integer = Integer::from(&r0 % &r1);
        if r.cmp(&Integer::from(0)) == cmp::Ordering::Equal {
            break;
        } else {
            r0 = r1;
            r1 = r;
        }
    }
    r1
}

// returns a list of divisors using the Euclidean Algorithm
fn euclidean_alg(a: &Integer, b: &Integer) -> Vec<Integer> {
    let mut v: Vec<Integer> = Vec::new();
    let mut r0: Integer = if a > b {a.clone()} else {b.clone()};
    let mut r1: Integer = if a > b {b.clone()} else {a.clone()};
    loop {
        let (q, r) = r0.div_rem(r1.clone());
        v.push(q);
        if r.cmp(&Integer::from(0)) == cmp::Ordering::Equal {
            break;
        } else {
            r0 = r1;
            r1 = r;
        }
    }
    v
}

// assume a and b are relatively prime, returns solution u and v
fn ext_euclidean_alg(a: &Integer, b: &Integer) -> (Integer, Integer) {
    let q = euclidean_alg(&a, &b);
    let mut p1: Integer = (&q[0]).clone();
    let mut p2: Integer = Integer::from(&q[1] * &p1 + &Integer::from(1));
    let mut q1: Integer = Integer::from(1);
    let mut q2: Integer = Integer::from(&q[1] * &q1);
    let len = q.len();
    for i in 2..len {
        let temp = p2.clone();
        p2 = Integer::from(&q[i] * &p2 + &p1);
        p1 = temp;
        let temp = q2.clone();
        q2 = Integer::from(&q[i] * &q2 + &q1);
        q1 = temp;
    }
    let parity = if len % 2 == 0 {1} else {-1};
    (q1 * parity, p1 * parity * -1)
}

fn main() {
    let z7_3 = Zp {
        p: Integer::from(7),
        value: Integer::from(3),
    };
    let z7_5 = Zp {
        p: Integer::from(7),
        value: Integer::from(5),
    };
    let zbig_2 = Zp {
        p: Integer::from(29996224275833i64),
        value: Integer::from(999999999989i64)
    };
    println!("3 + 5 mod 7 is {:?}", z7_3.add(&z7_5));
    println!("3 - 5 mod 7 is {:?}", z7_3.subtract(&z7_5));
    println!("3 * 5 mod 7 is {:?}", z7_3.multiply(&z7_5));
    println!("gcd(2024, 748) = {}", gcd(&Integer::from(2024), &Integer::from(748)));
    println!("gcd(9000, 729) = {}", gcd(&Integer::from(9000), &Integer::from(729)));
    println!("gcd(73, 25) = {:?}", euclidean_alg(&Integer::from(73), &Integer::from(25)));
    //println!("The inverse of 5 mod 7 is {:?}", z7_5.native_inverse());
    //println!("The inverse of 2 mod 123456789011 is {:?}", zbig_2.native_inverse());
    let (u, v) = ext_euclidean_alg(&Integer::from(73), &Integer::from(25));
    println!("u = {0}, v = {1}", u, v);
    //println!("inverse of 999,999,999,989 mod 29,996,224,275,833 is {:?}", zbig_2.native_inverse());
}
