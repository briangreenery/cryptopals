#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;
use std::collections::{HashMap, HashSet};

fn parse_cookie(cookie: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    for pair in cookie.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();
        result.insert(parts[0].to_string(), parts[1].to_string());
    }

    result
}

#[test]
fn parse_cookie_test() {
    let parsed = parse_cookie("foo=bar&baz=qux&zap=zazzle");

    assert_eq!(parsed.get("foo").unwrap(), "bar");
    assert_eq!(parsed.get("baz").unwrap(), "qux");
    assert_eq!(parsed.get("zap").unwrap(), "zazzle");
}

fn profile_for(email: &str) -> String {
    if email.contains("=") || email.contains("&") {
        panic!("bad email");
    }

    "email=".to_string() + email + "&uid=10&role=user"
}

#[test]
fn profile_for_test() {
    assert_eq!(profile_for("foo@bar.com"),
               "email=foo@bar.com&uid=10&role=user");
}

fn print_profile(profile: &HashMap<String, String>) {
    println!("email: {}, uid: {}, role: {}",
             profile.get("email").unwrap(),
             profile.get("uid").unwrap(),
             profile.get("role").unwrap());
}

struct Oracle {
    key: Vec<u8>,
}

impl Oracle {
    fn new() -> Oracle {
        let mut key = [0; 16];
        rand::thread_rng().fill_bytes(&mut key);

        Oracle { key: key.to_vec() }
    }

    fn encrypt(&self, email: &str) -> Vec<u8> {
        pals::aes::encrypt_ecb(profile_for(email).as_bytes(), &self.key)
    }

    fn decrypt(&self, cipher: &[u8]) -> HashMap<String, String> {
        let plain = pals::aes::decrypt_ecb(cipher, &self.key);
        parse_cookie(&String::from_utf8(plain).unwrap())
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
    count_duplicates(&oracle.encrypt(&String::from_utf8(data.to_vec()).unwrap())) > 10
}

fn determine_sizes(oracle: &Oracle) -> (usize, usize, usize) {
    let data = [b'A'; 1024];
    let start_len = oracle.encrypt("").len();

    for i in 1..data.len() {
        let len = oracle.encrypt(&String::from_utf8(data[0..i].to_vec()).unwrap()).len();

        if len != start_len {
            let block_size = len - start_len;
            let padding_size = i;
            let secret_size = start_len - padding_size;

            return (block_size, secret_size, padding_size);
        }
    }

    panic!("you bug");
}

fn get_admin_block(oracle: &Oracle, block_size: usize) -> Vec<u8> {
    let emaileq_len = "email=".len();
    let admin_len = "admin".len();

    let mut address = String::new();

    while emaileq_len + address.len() < block_size {
        address.push('y');
    }

    address.push_str("admin");
    let padding = (block_size - admin_len) as u8;

    while (emaileq_len + address.len()) % block_size != 0 {
        address.push(padding as char);
    }

    let encrypted = oracle.encrypt(&address);
    encrypted[block_size..2 * block_size].to_vec()
}

fn get_role_blocks(oracle: &Oracle, block_size: usize) -> Vec<u8> {
    let emaileq_len = "email=".len();
    let tail_len = "&uid=10&role=".len();
    let evil_len = "@evil.com".len();

    let mut address = String::new();
    address.push('x');

    while (emaileq_len + address.len() + evil_len + tail_len) % block_size != 0 {
        address.push('x');
    }

    address.push_str("@evil.com");

    let encrypted = oracle.encrypt(&address);
    encrypted[0..emaileq_len + address.len() + tail_len].to_vec()
}

fn main() {
    let oracle = Oracle::new();

    if is_ecb(&oracle) {
        println!("It's ECB (of course).");
    } else {
        panic!("you bug");
    }

    let (block_size, _secret_size, _padding_size) = determine_sizes(&oracle);
    println!("The block size is {}.", block_size);

    let admin_block = get_admin_block(&oracle, block_size);
    let addr_block = get_role_blocks(&oracle, block_size);

    let mut evil = Vec::new();

    evil.extend(addr_block);
    evil.extend(admin_block);

    let plain = oracle.decrypt(&evil);
    print_profile(&plain);
}
