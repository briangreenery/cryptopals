#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

struct CTR {
    key: Vec<u8>,
    nonce: i64,
    count: i64,
}

impl CTR {
    fn new(key: &[u8], nonce: i64) -> CTR {
        CTR {
            key: key.to_vec(),
            nonce: nonce,
            count: 0,
        }
    }

    fn next(&mut self) -> Vec<u8> {
        let mut block: [u8; 16] = [0; 16];

        for i in 0..8 {
            block[i] = ((self.nonce >> (8 * i)) & 0xff) as u8;
        }

        for i in 0..8 {
            block[8 + i] = ((self.count >> (8 * i)) & 0xff) as u8;
        }

        let mut result = Vec::new();
        pals::aes::encrypt_block(&block, &self.key, &mut result);
        self.count += 1;

        result
    }

    fn apply(&mut self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();

        for block in data.chunks(16) {
            let mut next = self.next();
            pals::aes::xor(block, &mut next);
            result.extend(&next[0..block.len()]);
        }

        result
    }
}

fn main() {
    let data = pals::base64::decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX\
                                     0KSvoOLSFQ==")
                   .unwrap();

    let key = b"YELLOW SUBMARINE";
    let plain = CTR::new(key, 0).apply(&data);

    println!("{}", String::from_utf8(plain).unwrap());
}
