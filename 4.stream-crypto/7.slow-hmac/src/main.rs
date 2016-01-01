#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    println!("{}", pals::hex::encode(&pals::sha1::hmac(b"key", b"hodor")));
}
