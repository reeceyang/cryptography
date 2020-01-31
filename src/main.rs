use rug::Integer;

#[derive(Debug)]
struct Zp {
    p: Integer,
    value: Integer
}

// this can probably be cleaned up
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
}

// Do we want gcd in Zp? Or in general?
fn gcd()

/*fn euclidean_algorithm(a: Zp, b: Zp) -> (Zp, Zp) {
    (a, b)
}*/

fn main() {
    let z7_3 = Zp {
        p: Integer::from(7),
        value: Integer::from(3),
    };
    let z7_5 = Zp {
        p: Integer::from(7),
        value: Integer::from(5),
    };
    println!("3 + 5 mod 7 is {:?}", z7_3.add(&z7_5));
    println!("3 - 5 mod 7 is {:?}", z7_3.subtract(&z7_5));
    println!("3 * 5 mod 7 is {:?}", z7_3.multiply(&z7_5));
}
