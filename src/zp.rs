use rug::Integer;
use std::cmp;
use rand::Rng;

#[derive(Debug)]
pub struct Zp {
    pub p: Integer,
    pub value: Integer
}

// this can probably be cleaned up - do we actually need all the clones?
impl Zp {
    pub fn add(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            panic!("Error: cannot add two Zps with a different p");
        }
        Zp {p: self.p.clone(), value: (self.value.clone() + a.value.clone()) % self.p.clone()}
    }
    pub fn subtract(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            panic!("Error: cannot subtract two Zps with a different p");
        }
        Zp {p: self.p.clone(), value: (self.value.clone() - a.value.clone() + self.p.clone()) % self.p.clone()}
    }
    pub fn multiply(&self, a: &Zp) -> Zp {
        if a.p != self.p {
            panic!("Error: cannot multiply two Zps with a different p");
        }
        Zp {p: self.p.clone(), value: (self.value.clone() * a.value.clone()) % self.p.clone()}
    }
    // finds the inverse using the rug crate's built in inverse function
    pub fn native_inverse(&self) -> Zp {
        let inv = match self.value.clone().invert(&self.p) {
            Ok(inverse) => inverse,
            Err(_) => unreachable!(),
        };
        Zp {p: self.p.clone(), value: inv}
    }
    // my own implementation of the inverse function
    pub fn inverse(&self) -> Zp {
        let (_n, inv) = ext_euclidean_alg(&self.value, &self.p);
        let inv = (inv + self.p.clone()) % self.p.clone();
        Zp {p: self.p.clone(), value: inv}
    }

    pub fn power(&self, mut A: Integer) -> Zp {
        let mut a = self.value.clone();
        let mut b = Integer::from(1);
        //println!("a: {0} b: {1} A: {2}", a, b, A);
        while A > 0 {
            if &A % Integer::from(2) == Integer::from(1) {
                b = (b * &a) % &self.p;
            }
            a = Integer::from(&a * &a) % &self.p;
            A /= Integer::from(2);
            //println!("a: {0} b: {1} A: {2}", a, b, A);
        }
        Zp {p: self.p.clone(), value: b}
    }
}

pub fn inverse(a: &Integer, p: &Integer) -> Integer {
    let (_n, inv) = ext_euclidean_alg(a, p);
    let inv = (inv + p.clone()) % p.clone();
    inv
}

// returns the gcd of two Integers using the Euclidean Algorithm
// could possibly use the euclidean_alg function to calculate this
pub fn gcd(a: &Integer, b: &Integer) -> Integer {
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

// if we don't want to return a Zp
pub fn power(n: &Integer, A1: &Integer, p: &Integer) -> Integer {
    let mut a = n.clone();
    let mut A = A1.clone();
    let mut b = Integer::from(1);
    while A > 0 {
        if &A % Integer::from(2) == Integer::from(1) {
            b = (b * &a) % p;
        }
        a = Integer::from(&a * &a) % p;
        A /= Integer::from(2);
    }
    b
}

// returns a list of divisors using the Euclidean Algorithm
pub fn euclidean_alg(a: &Integer, b: &Integer) -> Vec<Integer> {
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

// assume a and b are relatively prime, returns solution u and v to ua+vb=1
pub fn ext_euclidean_alg(a: &Integer, b: &Integer) -> (Integer, Integer) {
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

pub fn is_prime(n: &Integer) -> bool {
    let mut is_prime = true;
    for i in 1..300 {
        if miller_rabin(n, &generate_candidate_witness()) {
            is_prime = false;
            break;
        }
    }
    is_prime
}

// returns true if composite
pub fn miller_rabin(n: &Integer, a1: &Integer) -> bool {
    let mut a = a1.clone();
    if n % Integer::from(2) == Integer::from(0) || (1 < gcd(&a, &n) && &gcd(&a, &n) < n) {
        return true;
    } else {
        let (k, q) = find_k(&Integer::from(n - 1));
        let mut a = power(&a, &q, n);
        if Integer::from(&a % n).cmp(&Integer::from(1)) == cmp::Ordering::Equal {
            return false;
        } else {
            let mut i: Integer = Integer::from(0);
            while i < Integer::from(&k - 1) {
                if Integer::from(&a % n).cmp(&Integer::from(n - 1)) == cmp::Ordering::Equal {
                    return false;
                }
                a = Integer::from(&a * &a) % n;
            }
        }
    }
    true
}

// given n returns (k, q) such that 2^k
pub fn find_k(n: &Integer) -> (Integer, Integer) {
    let mut k = Integer::from(0);
    let mut q = n.clone();
    while Integer::from(&k % 2) == Integer::from(0) {
        q /= 2;
        k += 1;
    }
    (k, q)
}

// generate a candidate witness
fn generate_candidate_witness() -> Integer {
    Integer::from(9699690) * Integer::from(rand::thread_rng().gen_range(1, 9699690)) + 1
}
