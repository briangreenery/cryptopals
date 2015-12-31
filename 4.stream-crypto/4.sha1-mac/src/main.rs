#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    let hash = pals::sha1::hash(b"hodor");
    println!("{}", pals::hex::encode(&hash));
}
