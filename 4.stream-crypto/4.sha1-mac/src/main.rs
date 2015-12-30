#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    let hash = pals::sha1::hash(b"Hodor");
    println!("{}", pals::hex::encode(&hash));
}
