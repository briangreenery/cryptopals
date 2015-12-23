#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;
extern crate time;

use rand::Rng;

struct MT19937Stream {
    mt: pals::MT19937,
    buffer: [u8; 4],
    index: usize,
}

impl MT19937Stream {
    fn new(seed: u16) -> MT19937Stream {
        MT19937Stream {
            mt: pals::MT19937::new(seed as u32),
            buffer: [0; 4],
            index: 4,
        }
    }

    fn get(&mut self) -> u8 {
        if self.index == self.buffer.len() {
            let next = self.mt.gen();

            for i in 0..self.buffer.len() {
                self.buffer[i] = ((next >> (8 * i)) & 0xff) as u8;
            }

            self.index = 0;
        }

        let byte = self.buffer[self.index];
        self.index += 1;
        byte
    }
}

fn apply(data: &[u8], key: u16) -> Vec<u8> {
    let mut stream = MT19937Stream::new(key);
    let mut result = Vec::new();

    for byte in data.iter() {
        result.push(byte ^ stream.get());
    }

    result
}

fn encrypt_with_random_prefix(data: &[u8], key: u16) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut plain = Vec::new();

    let prefix_len = rng.gen_range(0, 50);
    let mut buffer = [0; 64];
    rng.fill_bytes(&mut buffer[..prefix_len]);

    plain.extend(&buffer[..prefix_len]);
    plain.extend(data);
    apply(&plain, key)
}

fn guess_key(cipher: &[u8], known: &[u8]) -> u16 {
    for i in 0..0xFFFF {
        let key = i as u16;
        let decrypted = apply(&cipher, key);

        if &decrypted[decrypted.len() - known.len()..] == known {
            return key;
        }
    }

    panic!("no key found!");
}

fn now_unix_time() -> u32 {
    time::now_utc().to_timespec().sec as u32
}

fn create_password_token(user: &str) -> Vec<u8> {
    let key = (now_unix_time() & 0xFFFF) as u16;

    let mut plain = Vec::new();
    plain.extend(user.as_bytes());
    plain.extend(b":password reset token");

    apply(&plain, key)
}

fn main() {
    let random_key = rand::thread_rng().gen();
    let cipher = encrypt_with_random_prefix(b"AAAAAAAAAAAAAA", random_key);
    let guessed_key = guess_key(&cipher, b"AAAAAAAAAAAAAA");

    println!("key was {}", random_key);
    println!("key guessed to be {}", guessed_key);

    let token = create_password_token("hodor");
    let token_key = guess_key(&token, b":password reset token");

    let now = (now_unix_time() & 0xFFFF) as u16;
    if token_key == now || token_key.wrapping_add(1) == now {
        println!("the token was created now");
    } else {
        println!("the token was not created now");
    }
}
