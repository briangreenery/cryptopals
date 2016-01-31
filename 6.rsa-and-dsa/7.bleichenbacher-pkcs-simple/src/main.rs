#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use rand::Rng;

struct RSA {
    n: BigNum,
    e: BigNum,
    d: BigNum,
    size: usize,
}

impl RSA {
    fn new(p: &BigNum, q: &BigNum) -> RSA {
        let one = BigNum::new(1);
        let n = p.mul(&q);
        let et = p.sub(&one).mul(&q.sub(&one));
        let e = BigNum::new(3);
        let d = e.modinv(&et);
        let size = n.to_bytes().len();
        RSA {
            n: n,
            e: e,
            d: d,
            size: size,
        }
    }

    fn encrypt(&self, plain: &BigNum) -> BigNum {
        plain.modexp(&self.e, &self.n)
    }

    fn decrypt(&self, cipher: &BigNum) -> BigNum {
        cipher.modexp(&self.d, &self.n)
    }
}

fn pkcs15_pad(data: &[u8], size: usize) -> Vec<u8> {
    let mut padding = vec![0; size];

    if data.len() + 11 > size {
        panic!("data too large");
    }

    padding[1] = 2;

    let mut rng = rand::thread_rng();
    for i in 2..size - data.len() - 1 {
        while padding[i] == 0 {
            padding[i] = rng.gen()
        }
    }

    for (i, byte) in data.iter().enumerate() {
        padding[size - data.len() + i] = *byte;
    }

    padding
}

fn pkcs15_unpad(padding: &[u8]) -> Option<&[u8]> {
    if padding.len() < 11 {
        return None;
    }

    if padding[0] != 0 {
        return None;
    }

    if padding[1] != 2 {
        return None;
    }

    for i in 2..10 {
        if padding[i] == 0 {
            return None;
        }
    }

    for i in 10..padding.len() {
        if padding[i] == 0 {
            return Some(&padding[i + 1..]);
        }
    }

    return None;
}

fn pkcs15_is_valid(key: &RSA, ciphertext: &BigNum) -> bool {
    let plain = key.decrypt(&ciphertext).to_bytes();

    let mut buffer = vec![0; key.size - plain.len()];
    buffer.extend(&plain);

    pkcs15_unpad(&buffer).is_some()
}

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("F29710A696210BD8C1FE4C6A5A22873D").unwrap());
    let q = BigNum::from_bytes(&pals::hex::decode("C268B715B7AF1C3312D9BAB054F710C9").unwrap());
    let key = RSA::new(&p, &q);

    let padded = pkcs15_pad("kick it, CC".as_bytes(), key.size);
    let ciphertext = key.encrypt(&BigNum::from_bytes(&padded));

    let is_valid = pkcs15_is_valid(&key, &ciphertext);
}
