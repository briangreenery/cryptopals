#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use rand::Rng;

fn random_exponent(prime: &BigNum) -> BigNum {
    let mut exponent_bytes = [0xff; 192];

    loop {
        rand::thread_rng().fill_bytes(&mut exponent_bytes);
        let exponent = BigNum::from_bytes(&exponent_bytes);

        if &exponent < prime {
            return exponent;
        }
    }
}

fn make_dh_key(prime: &BigNum) -> (BigNum, BigNum) {
    let exponent = random_exponent(&prime);
    let pubkey = BigNum::new(2).modexp(&exponent, &prime);
    (exponent, pubkey)
}

fn main() {
    let prime = BigNum::from_bytes(&pals::hex::decode("ffffffffffffffffc90fdaa22168c234c4c6628b8\
                                                       0dc1cd129024e088a67cc74020bbea63b139b2251\
                                                       4a08798e3404ddef9519b3cd3a431b302b0a6df25\
                                                       f14374fe1356d6d51c245e485b576625e7ec6f44c\
                                                       42e9a637ed6b0bff5cb6f406b7edee386bfb5a899\
                                                       fa5ae9f24117c4b1fe649286651ece45b3dc2007c\
                                                       b8a163bf0598da48361c55d39a69163fa8fd24cf5\
                                                       f83655d23dca3ad961c62f356208552bb9ed52907\
                                                       7096966d670c354e4abc9804f1746c08ca237327f\
                                                       fffffffffffffff")
                                        .unwrap());

    let (a_private, _) = make_dh_key(&prime);

    let a_session = prime.modexp(&a_private, &prime);

    let a_key = pals::sha1::hash(&a_session.to_bytes());
    let a_iv = pals::aes::random_key();

    let evil_key = pals::sha1::hash(&[]);

    let encrypted = pals::aes::encrypt_cbc(b"Hello, world!", &a_key[0..16], &a_iv);
    let evil_intercepted = pals::aes::decrypt_cbc(&encrypted, &evil_key[0..16], &a_iv);

    println!("intercepted: {}",
             String::from_utf8(evil_intercepted).unwrap());
}
