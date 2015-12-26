#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;

struct Oracle {
    ctr: pals::aes::CTR,
}

impl Oracle {
    fn new() -> Oracle {
        Oracle { ctr: pals::aes::CTR::new(&pals::aes::random_key(), rand::thread_rng().gen()) }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut plain = Vec::new();

        if data.contains(&b';') || data.contains(&b'=') {
            panic!("invalid user data");
        }

        plain.extend(b"comment1=cooking%20MCs;userdata=".iter());
        plain.extend(data);
        plain.extend(b";comment2=%20like%20a%20pound%20of%20bacon".iter());
        self.ctr.apply(&plain)
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.ctr.apply(data)
    }
}

fn is_admin(data: &[u8]) -> bool {
    let admin = b";admin=true;";

    if data.len() < admin.len() {
        return false;
    }

    for start in 0..data.len() - admin.len() + 1 {
        let mut found = true;

        for i in 0..admin.len() {
            if data[start + i] != admin[i] {
                found = false;
                break;
            }
        }

        if found {
            return true;
        }
    }

    false
}

#[test]
fn is_admin_test() {
    assert!(is_admin(b";admin=true;"));
    assert!(!is_admin(b";adminxtrue;"));
    assert!(is_admin(b"foo;admin=true;bar"));
    assert!(is_admin(b"foo;admin=true;"));
    assert!(!is_admin(b"foo;admin=true"));
}

fn main() {
    let oracle = Oracle::new();

    let mut encrypted = oracle.encrypt(b"hodor@example.comAAAAAAAAAAA");
    let target = b";admin=true";

    for i in 0..target.len() {
        encrypted[49 + i] ^= b'A' ^ target[i];
    }

    let decrypted = oracle.decrypt(&encrypted);

    println!("Decrypted blocks:");
    println!("----------");

    for chunk in decrypted.chunks(16) {
        let string = String::from_utf8(chunk.to_vec());

        match string {
            Ok(value) => println!("{}", value),
            Err(_) => println!("<garbage={}>", pals::hex::encode(chunk)),
        };
    }

    println!("----------");
    println!("admin={}", is_admin(&decrypted));
}
