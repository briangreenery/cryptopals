#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;
use std::collections::{HashSet, HashMap};

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
        let mut plain = Vec::new();
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

fn code_book(oracle: &Oracle, decrypted: &[u8], block_size: usize) -> HashMap<Vec<u8>, u8> {
    let mut book = HashMap::new();

    let mut data = Vec::new();
    data.push(0);

    if decrypted.len() >= block_size {
        data.extend(&decrypted[0..block_size - 1]);
    } else {
        data.extend(decrypted);
        let padding = block_size - data.len();

        while data.len() < block_size {
            data.push(padding as u8);
        }
    }

    for i in 0..256 {
        let byte = i as u8;
        data[0] = byte;

        let result = oracle.encrypt(&data);
        book.insert(result[0..block_size].to_vec(), byte);
    }

    book
}

fn main() {
    let oracle = Oracle::new();

    if is_ecb(&oracle) {
        println!("It's ECB (of course).");
    } else {
        panic!("you bug");
    }

    let (block_size, secret_size, padding_size) = determine_sizes(&oracle);

    println!("The block size is {}.", block_size);
    println!("The secret size is {}.", secret_size);
    println!("The padding size is {}.", padding_size);

    let mut offset = (padding_size % block_size) + 1;
    let mut decrypted = Vec::new();

    for _i in 0..secret_size {
        let book = code_book(&oracle, &decrypted, block_size);

        let mut garbage = Vec::new();
        while garbage.len() < offset {
            garbage.push(b'A');
        }

        let result = oracle.encrypt(&garbage);

        let block_start = offset + secret_size - decrypted.len() - 1;
        let block_end = block_start + block_size;
        let last = &result[block_start..block_end];

        match book.get(last) {
            Some(letter) => {
                decrypted.insert(0, *letter);
            }
            None => {
                panic!("you bug");
            }
        }

        println!("{}", String::from_utf8(decrypted.clone()).unwrap());
        println!("----------");

        offset += 1;
    }
}
