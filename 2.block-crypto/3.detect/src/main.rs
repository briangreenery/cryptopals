#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;
use std::collections::HashSet;

fn encryption_oracle(data: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let prepend_data = b"I'm back and I'm ringin' the bell";
    let append_data = b"Vanilla's on the mike, man I'm not lazy.";

    let mut plain: Vec<u8> = Vec::new();

    let prepend_amount = rng.gen_range(5, 10);
    let append_amount = rng.gen_range(5, 10);

    plain.extend(&prepend_data[..prepend_amount]);
    plain.extend(data);
    plain.extend(&append_data[..append_amount]);

    let mut key = [0; 16];
    rng.fill_bytes(&mut key);

    let mut iv = [0; 16];
    rng.fill_bytes(&mut iv);

    if rng.gen() {
        println!("using ecb");
        pals::aes::encrypt_ecb(&plain, &key)
    } else {
        println!("using cbc");
        pals::aes::encrypt_cbc(&plain, &key, &iv)
    }
}

fn count_duplicates(cipher: &[u8]) -> usize {
    let mut seen = HashSet::new();
    let mut count = 0;

    for block in cipher.chunks(16) {
        if seen.contains(block) {
            count += 1;
        }

        seen.insert(block);
    }

    count
}

fn main() {
    for _i in 0..100 {
        let plain = [b'A'; 1024];
        let encrypted = encryption_oracle(&plain);

        if count_duplicates(&encrypted) > 10 {
            println!("guess ecb");
        } else {
            println!("guess cbc");
        }

        println!("");
    }
}
