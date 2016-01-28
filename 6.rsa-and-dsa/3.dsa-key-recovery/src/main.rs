#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;

fn dsa_private_key(q: &BigNum, rinv: &BigNum, s: &BigNum, h: &BigNum, k: &BigNum) -> BigNum {
    let mut sk = s.mul(&k);

    while &sk < h {
        sk = sk.add(&q);
    }

    sk.sub(&h).mul(&rinv).div(&q).1
}

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

    let text = "For those that envy a MC it can be hazardous to your health\nSo be friendly, a \
                matter of life and death, just like a etch-a-sketch\n";

    let r = BigNum::from_decimal("548099063082341131477253921760299949438196259240");
    let s = BigNum::from_decimal("857042759984254168557880549501802188789837994940");
    let h = BigNum::from_bytes(&pals::sha1::hash(text.as_bytes()));

    let y = BigNum::from_bytes(&pals::hex::decode("084ad4719d044495496a3201c8ff484feb45b962e7302\
                                                   e56a392aee4abab3e4bdebf2955b4736012f21a080840\
                                                   56b19bcd7fee56048e004e44984e2f411788efdc837a0\
                                                   d2e5abb7b555039fd243ac01f0fb2ed1dec568280ce67\
                                                   8e931868d23eb095fde9d3779191b8c0299d6e07bbb28\
                                                   3e6633451e535c45513b2d33c99ea17")
                                    .unwrap());

    let rinv = r.modinv(&q);

    for k in 1..65536 {
        let x = dsa_private_key(&q, &rinv, &s, &h, &BigNum::new(k));
        let y2 = g.modexp(&x, &p);

        if y == y2 {
            println!("private key is: {}", pals::hex::encode(&x.to_bytes()));
            break;
        }

        if k % 1024 == 0 {
            println!("k = {}", k);
        }
    }
}
