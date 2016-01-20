#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    let num = pals::BigNum::new(2);

    let mut product = pals::BigNum::new(1);

    for i in 1..1001 {
        product = product.mul(&num);
        println!("{} = {}", i, product.to_decimal());
    }
}
