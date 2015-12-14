#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;

fn random_string() -> Vec<u8> {
    let strings = ["MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
                   "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
                   "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
                   "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
                   "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
                   "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
                   "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
                   "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
                   "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
                   "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"];

    let index: usize = rand::thread_rng().gen_range(0, strings.len());
    pals::base64::decode(strings[index]).unwrap()
}

struct Oracle {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        let mut key = [0; 16];
        rand::thread_rng().fill_bytes(&mut key);

        let mut iv = [0; 16];
        rand::thread_rng().fill_bytes(&mut iv);

        Oracle {
            key: key.to_vec(),
            iv: iv.to_vec(),
        }
    }

    fn encrypt(&self) -> (Vec<u8>, Vec<u8>) {
        let string = random_string();
        let encrypted = pals::aes::encrypt_cbc(&string, &self.key, &self.iv);

        (encrypted, self.iv.clone())
    }

    fn decrypt(&self, data: &[u8]) -> bool {
        let mut result = Vec::new();
        let mut last = self.iv.to_vec();

        for block in data.chunks(16) {
            pals::aes::decrypt_block(block, &self.key, &mut result);

            let start = result.len() - 16;
            let end = result.len();

            pals::aes::xor(&last, &mut result[start..end]);
            last = block.to_vec();
        }

        pals::aes::is_valid_padding(&result[result.len() - 16..])
    }
}

fn decrypt_block(oracle: &Oracle, block: &[u8], last: &[u8]) -> Vec<u8> {
    let mut buffer = [0; 32];

    for i in 0..16 {
        buffer[i + 16] = block[i];
    }

    let mut candidates: Vec<Vec<u8>> = Vec::new();
    candidates.push(Vec::new());

    loop {
        let mut next_candidates = Vec::new();

        for candidate in candidates {
            let padding = candidate.len() + 1;

            for i in 0..16 {
                buffer[i] = last[i];
            }

            for guess in 0..256 {
                let byte = guess as u8;
                buffer[16 - padding] = byte ^ (padding as u8);

                for i in 0..candidate.len() {
                    buffer[16 - candidate.len() + i] = candidate[i] ^ (padding as u8);
                }

                for i in 16 - padding..16 {
                    buffer[i] ^= last[i];
                }

                if oracle.decrypt(&buffer) {
                    let mut next_candidate = Vec::new();
                    next_candidate.push(byte);
                    next_candidate.extend(&candidate);
                    next_candidates.push(next_candidate);
                }
            }
        }

        if next_candidates.len() == 0 {
            panic!("decryption failed");
        }

        candidates = next_candidates;
        if candidates[0].len() == 16 {
            break;
        }
    }

    candidates[0].clone()
}

fn main() {
    let oracle = Oracle::new();

    let (encrypted, iv) = oracle.encrypt();
    let mut last = iv;
    let mut decrypted = Vec::new();

    for block in encrypted.chunks(16) {
        decrypted.extend(decrypt_block(&oracle, block, &last));
        last = block.to_vec();
    }

    let size_without_padding = decrypted.len() - (decrypted[decrypted.len() - 1] as usize);
    decrypted.truncate(size_without_padding);

    println!("{}", String::from_utf8(decrypted).unwrap());
}
