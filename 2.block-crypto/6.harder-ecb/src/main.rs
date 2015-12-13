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

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        gcd(b, a)
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(144, 160), 16);
}

fn guess_block_size(oracle: &Oracle) -> usize {
    let data = [b'A'; 32];
    let start_len = oracle.encrypt(&[]).len();
    let mut diffs = Vec::new();

    while diffs.len() < 8 {
        for i in 1..32 {
            let len = oracle.encrypt(&data[0..i]).len();

            let diff = if len > start_len {
                len - start_len
            } else {
                start_len - len
            };

            if diff != 0 {
                diffs.push(diff);
            }
        }
    }

    diffs.iter().fold(diffs[0], |accum, &item| gcd(accum, item))
}

fn get_marker(oracle: &Oracle, block_size: usize) -> Vec<u8> {
    let encrypted = oracle.encrypt(&[b'A'; 1024]);
    let mut seen = HashMap::new();

    for block in encrypted.chunks(block_size) {
        *seen.entry(block).or_insert(0) += 1
    }

    for (block, count) in seen {
        if count > 10 {
            return block.to_vec();
        }
    }

    panic!("you bug");
}

fn target_data(decrypted: &[u8], block_size: usize) -> Vec<u8> {
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

    data
}

fn code_book(oracle: &Oracle,
             decrypted: &[u8],
             block_size: usize,
             marker: &[u8])
             -> HashMap<Vec<u8>, u8> {
    let mut book = HashMap::new();

    let mut data = target_data(&decrypted, block_size);

    for i in 0..256 {
        let byte = i as u8;
        data[0] = byte;

        let mut plain = Vec::new();

        for _j in 0..2 * block_size {
            plain.push(b'A');
        }

        for _j in 0..2 * block_size {
            plain.extend(&data);
            plain.push(b'x');
        }

        for _j in 0..2 * block_size {
            plain.push(b'A');
        }

        let encrypted = oracle.encrypt(&plain);

        let mut start: Option<usize> = None;

        for (index, chunk) in encrypted.chunks(block_size).enumerate() {
            if start.is_none() {
                if chunk == marker {
                    start = Some(index);
                }
            } else if chunk == marker {
                if index != start.unwrap() + 1 {
                    break;
                }
            } else {
                book.insert(chunk.to_vec(), byte);
            }
        }
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

    let block_size = guess_block_size(&oracle);
    println!("Block size is {}.", block_size);

    let marker = get_marker(&oracle, block_size);
    println!("Marker is {}", pals::hex::encode(&marker));

    let mut decrypted = Vec::new();

    loop {
        let book = code_book(&oracle, &decrypted, block_size, &marker);
        let decrypted_len = decrypted.len();

        while decrypted_len == decrypted.len() {
            for offset in 0..2 * block_size {
                let mut garbage = Vec::new();

                while garbage.len() < offset {
                    garbage.push(b'A');
                }

                for _i in 0..2 * block_size {
                    garbage.push(b'X');
                }

                let result = oracle.encrypt(&garbage);

                let block_start = result.len() -
                                  block_size * (1 + ((decrypted.len() + 1) / block_size));
                let block_end = block_start + block_size;

                let target = &result[block_start..block_end];

                if let Some(letter) = book.get(target) {
                    decrypted.insert(0, *letter);
                    break;
                }
            }
        }

        println!("{}", String::from_utf8(decrypted.clone()).unwrap());

        if decrypted.len() >= 16 && decrypted[..8] == [b'X'; 8] {
            println!("{}", String::from_utf8(decrypted[8..].to_vec()).unwrap());
            break;
        }
    }
}
