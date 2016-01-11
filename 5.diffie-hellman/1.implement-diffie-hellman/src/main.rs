#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;

fn mac(key: &[u8], message: &[u8]) -> [u8; 16] {
    let mut hasher = pals::md4::Hasher::new();

    hasher.write(key);
    hasher.write(message);
    hasher.end()
}

fn random_key() -> Vec<u8> {
    let mut buffer = [0; 64];
    let prefix_len = rand::thread_rng().gen_range(8, 64);
    rand::thread_rng().fill_bytes(&mut buffer[..prefix_len]);
    buffer[..prefix_len].to_vec()
}

fn md4_pad(message: &[u8], key_len: usize) -> Vec<u8> {
    let mut padded = Vec::new();

    padded.extend(message);
    padded.push(0x80);

    while (key_len + padded.len() + 8) % 64 != 0 {
        padded.push(0x00);
    }

    let bit_len = (8 * (key_len + message.len())) as u64;

    for i in 0..8 {
        padded.push(((bit_len >> (8 * i)) % 256) as u8);
    }

    padded
}

fn main() {
    let key = random_key();
    println!("actual key length is {}", key.len());

    let message = b"comment1=cooking%20MCs;userdata=foo;comment2=%20like%20a%20pound%20of%20bacon";
    let hash = mac(&key, message);

    for key_len in 0..1000 {
        let padded = md4_pad(message, key_len);
        let admin = b";admin=true";

        let mut forged = Vec::new();
        forged.extend(&padded);
        forged.extend(admin);

        let mut forger = pals::md4::Hasher::from(&hash, key_len + padded.len());
        forger.write(admin);
        let forged_hash = forger.end();

        if mac(&key, &forged) == forged_hash {
            println!("guess key length is {}", key_len);
            println!("forged message: {}", pals::hex::encode(&forged));
            break;
        }
    }
}
