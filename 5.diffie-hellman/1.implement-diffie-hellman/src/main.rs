#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    let one = pals::BigNum::from_decimal("1");

    let mut factorial = pals::BigNum::from_decimal("1");
    let mut number = pals::BigNum::from_decimal("1");

    for i in 0..101 {
        println!("{}! = {}", i, factorial.to_decimal());
        factorial = factorial.mul(&number);
        number = number.add(&one);
    }
}
