#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use pals::hex::decode;

fn divide(a: &BigNum, b: &BigNum) {
    let (q, r) = a.div(&b);
    println!("{} / {} = {} remainder {}",
             a.to_decimal(),
             b.to_decimal(),
             q.to_decimal(),
             r.to_decimal());
}

fn main() {
    let a = BigNum::from_bytes(&decode("800000000000000000000003").unwrap());
    let b = BigNum::from_bytes(&decode("200000000000000000000001").unwrap());

    divide(&a, &b);
}
