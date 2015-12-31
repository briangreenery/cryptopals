#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn mac(key: &[u8], message: &[u8]) -> [u8; 20] {
    let mut hasher = pals::sha1::Hasher::new();

    hasher.write(key);
    hasher.write(message);
    hasher.end()
}

fn is_valid(key: &[u8], message: &[u8], hash: [u8; 20]) -> bool {
    mac(key, message) == hash
}

fn main() {
    let key = b"hodor";
    let message = b"cats are cute";

    let hash = mac(key, message);
    println!("{}", is_valid(key, message, hash));
}
