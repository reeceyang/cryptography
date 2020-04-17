use rug::Integer;
use std::cmp;
use rand::Rng;
use std::time::SystemTime;
use crate::zp;

pub fn is_prime(n: &Integer) -> bool {
    let mut is_prime = true;
    for _i in 1..30 {
        if miller_rabin(n, &generate_candidate_witness()) {
            is_prime = false;
            break;
        }
    }
    is_prime
}

// returns true if composite
pub fn miller_rabin(n: &Integer, a1: &Integer) -> bool {
    let a = a1.clone();
    if n % Integer::from(2) == Integer::from(0) || (1 < zp::gcd(&a, &n) && &zp::gcd(&a, &n) < n) {
        //println!("first condition - composite");
        return true;
    } else {
        let (k, q) = find_k(&Integer::from(n - 1));
        let mut a = zp::power(&a, &q, n);
        if Integer::from(&a % n).cmp(&Integer::from(1)) == cmp::Ordering::Equal {
            //println!("a equiv 1 mod n - test fails");
            return false;
        } else {
            let mut i: Integer = Integer::from(0);
            while i <= Integer::from(&k - 1) {
                if Integer::from((a.clone() + Integer::from(1)) % n).cmp(&Integer::from(0)) == cmp::Ordering::Equal {
                    //println!("a equiv -1 mod n - test fails");
                    return false;
                }
                a = Integer::from(&a * &a) % n;
                i += 1;
            }
        }
    }
    //println!("end of loop - composite");
    true
}

// given n returns (k, q) such that 2^k
pub fn find_k(n: &Integer) -> (Integer, Integer) {
    let mut k = Integer::from(0);
    let mut q = n.clone();
    while Integer::from(&q % 2) == Integer::from(0) {
        q /= 2;
        k += 1;
    }
    //println!("n: {0}, k: {1}, q: {2}", n, k, q);
    (k, q)
}

// generate a candidate witness
fn generate_candidate_witness() -> Integer {
    let cand = Integer::from(9699690) * Integer::from(rand::thread_rng().gen_range(1, 9699690)) + 1;
    //println!("Candidate Witness: {}", cand);
    cand
}

// generate a random prime of length n bits
pub fn generate_random_prime(n: &usize) -> Integer {
    use rug::rand::RandState;
    loop {
        let mut rand = RandState::new();
        let mut seed = Integer::new();
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => seed = Integer::from(n.as_secs()),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        rand.seed(&seed);
        let mut candidate = Integer::from(rand.bits(32));
        candidate.set_bit(0, true);
        candidate.set_bit((n - 1) as u32, true);
        if is_prime(&candidate) {
            return candidate;
        }
    }
}
