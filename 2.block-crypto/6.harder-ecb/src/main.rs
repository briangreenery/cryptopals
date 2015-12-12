#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;
use std::collections::{HashMap, HashSet};

fn secret_data() -> Vec<u8> {
    pals::base64::decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4g\
                          YmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQg\
                          eW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK")
        .unwrap()
}

struct Oracle {
    key: Vec<u8>,
    secret: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        let mut key = [0; 16];
        rand::thread_rng().fill_bytes(&mut key);

        Oracle {
            key: key.to_vec(),
            secret: secret_data(),
        }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut plain = Vec::new();

        let prefix_len = rng.gen_range(0, 50);
        let mut buffer = [0; 64];
        rng.fill_bytes(&mut buffer[..prefix_len]);

        plain.extend(&buffer[..prefix_len]);
        plain.extend(data);
        plain.extend(&self.secret);
        pals::aes::encrypt_ecb(&plain, &self.key)
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

fn is_ecb(oracle: &Oracle) -> bool {
    let data = [b'A'; 1024];
    count_duplicates(&oracle.encrypt(&data)) > 10
}

fn determine_sizes(oracle: &Oracle) -> (usize, usize, usize) {
    let data = [b'A'; 1024];
    let start_len = oracle.encrypt(&[]).len();

    for i in 1..data.len() {
        let len = oracle.encrypt(&data[0..i]).len();

        if len != start_len {
            let block_size = len - start_len;
            let padding_size = i;
            let secret_size = start_len - padding_size;

            return (block_size, secret_size, padding_size);
        }
    }

    panic!("you bug");
}

fn main() {
    let oracle = Oracle::new();

    if is_ecb(&oracle) {
        println!("It's ECB (of course).");
    } else {
        panic!("you bug");
    }
}
