#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use pals::DSA;

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("800000000000000089e1855218a0e7dac38136ffafa72\
                                                   eda7859f2171e25e65eac698c1702578b07dc2a1076da\
                                                   241c76c62d374d8389ea5aeffd3226a0530cc565f3bf6\
                                                   b50929139ebeac04f48c3c84afb796d61e5a4f9a8fda8\
                                                   12ab59494232c7d2b4deb50aa18ee9e132bfa85ac4374\
                                                   d7f9091abc3d015efc871a584471bb1")
                                    .unwrap());

    let q = BigNum::from_bytes(&pals::hex::decode("f4f47f05794b256174bba6e9b396a7707e563c5b")
                                    .unwrap());

    let g = BigNum::from_bytes(&pals::hex::decode("5958c9d3898b224b12672c0b98e06c60df923cb8bc999\
                                                   d119458fef538b8fa4046c8db53039db620c094c9fa07\
                                                   7ef389b5322a559946a71903f990f1f7e0e025e2d7f7c\
                                                   f494aff1a0470f5b64c36b625a097f1651fe775323556\
                                                   fe00b3608c887892878480e99041be601a62166ca6894\
                                                   bdd41a7054ec89f756ba9fc95302291")
                                    .unwrap());

    {
        let mut key = DSA::new(p.clone(), q.clone(), g.clone());

        let text = "Cats are NOT cute";

        let z = BigNum::new(12345678);
        let r = key.public_key().modexp(&z, &p).div(&q).1;
        let s = r.mul(&z.modinv(&q)).div(&q).1;

        println!("r: {}", r.to_decimal());
        println!("s: {}", s.to_decimal());

        let signature = (r, s);

        key.set_g(&p.add(&BigNum::new(1)));
        println!("With g = p + 1, signature is valid: {}",
                 key.verify(text.as_bytes(), &signature));
    }
}
