#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use std::fmt;

struct RSA {
    n: BigNum,
    e: BigNum,
    d: BigNum,
}

impl RSA {
    fn new(p: &BigNum, q: &BigNum) -> RSA {
        let n = p.mul(&q);
        let et = p.sub(&BigNum::new(1)).mul(&q.sub(&BigNum::new(1)));
        let e = BigNum::new(3);
        let d = e.modinv(&et);
        RSA { n: n, e: e, d: d }
    }

    fn encrypt(&self, plain: &[u8]) -> BigNum {
        let size = self.n.to_bytes().len();

        if plain.len() >= size {
            panic!("text to encrypt is too big");
        }

        let mut bytes = plain.to_vec();
        while bytes.len() + 1 < size {
            bytes.push(0);
        }

        BigNum::from_bytes(&bytes).modexp(&self.e, &self.n)
    }

    fn decrypt(&self, cipher: &BigNum) -> BigNum {
        cipher.modexp(&self.d, &self.n)
    }
}

fn is_even(key: &RSA, cipher: &BigNum) -> bool {
    let number = key.decrypt(&cipher);

    if number.div(&BigNum::new(2)).1 == BigNum::new(0) {
        true
    } else {
        false
    }
}

struct Midpoint {
    whole: BigNum,
    numerator: BigNum,
    denominator: BigNum,
}

impl Midpoint {
    fn new(whole: &BigNum) -> Midpoint {
        Midpoint {
            whole: whole.clone(),
            numerator: BigNum::new(0),
            denominator: BigNum::new(1),
        }
    }

    fn div_by_2(&self) -> Self {
        let improper = self.whole.mul(&self.denominator).add(&self.numerator);
        let denominator = self.denominator.mul(&BigNum::new(2));

        let (q, r) = improper.div(&denominator);

        Midpoint {
            whole: q,
            numerator: r,
            denominator: denominator,
        }
    }

    fn add(&self, other: &Self) -> Self {
        let one = BigNum::new(1);
        let two = BigNum::new(2);

        let mut whole = self.whole.clone();
        let mut numerator = self.numerator.clone();
        let mut denominator = self.denominator.clone();

        assert!(denominator < other.denominator);

        while denominator != other.denominator {
            numerator = numerator.mul(&two);
            denominator = denominator.mul(&two);
        }

        whole = whole.add(&other.whole);
        numerator = numerator.add(&other.numerator);

        while numerator > denominator {
            whole = whole.add(&one);
            numerator = numerator.sub(&denominator);
        }

        Midpoint {
            whole: whole,
            numerator: numerator,
            denominator: denominator,
        }
    }

    fn floor(&self) -> BigNum {
        self.whole.clone()
    }

    fn ceiling(&self) -> BigNum {
        if self.numerator == BigNum::new(0) {
            self.whole.clone()
        } else {
            self.whole.add(&BigNum::new(1))
        }
    }
}

impl fmt::Display for Midpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} + {}/{}",
               self.whole.to_decimal(),
               self.numerator.to_decimal(),
               self.denominator.to_decimal())
    }
}

fn main() {
}
