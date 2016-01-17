#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    let mut num = pals::BigNum::from_bytes(&[1]);
    let two = pals::BigNum::from_bytes(&[2]);

    for i in 0..201 {
        println!("{}: {}", i, num.to_decimal());
        num = num.mul(&two);
    }
}
