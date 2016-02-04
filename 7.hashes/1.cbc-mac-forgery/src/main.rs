#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn cbc_mac_1(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let encrypted = pals::aes::encrypt_cbc(data, key, iv);
    encrypted[encrypted.len() - 16..].to_vec()
}

fn verify_request_1(key: &[u8], message: &str, iv: &[u8], mac: &[u8]) {
    if cbc_mac_1(message.as_bytes(), key, iv) == mac {
        let req = pals::cookie::parse(message);
        println!("Verified: Transfer ${} from {} to {}.",
                 req["amount"],
                 req["from"],
                 req["to"]);
    } else {
        println!("Invalid signature!");
    }
}

fn part1() {
    let key = pals::aes::random_key();

    let message = "from=totallyfine&to=evil&amount=1M";
    let iv = pals::aes::random_key();
    let mac = cbc_mac_1(message.as_bytes(), &key, &iv);
    verify_request_1(&key, message, &iv, &mac);

    let evil_message = "from=unsuspectin&to=evil&amount=1M";

    let mut evil_iv = [0; 16];
    pals::aes::xor(&iv, &mut evil_iv);
    pals::aes::xor(&message.as_bytes()[0..16], &mut evil_iv);
    pals::aes::xor(&evil_message.as_bytes()[0..16], &mut evil_iv);

    verify_request_1(&key, evil_message, &evil_iv, &mac);
}

fn cbc_mac_2(data: &[u8], key: &[u8]) -> Vec<u8> {
    let iv = [0; 16];
    let encrypted = pals::aes::encrypt_cbc(data, key, &iv);
    encrypted[encrypted.len() - 16..].to_vec()
}

fn verify_request_2(key: &[u8], message: &str, mac: &[u8]) {
    if cbc_mac_2(message.as_bytes(), key) == mac {
        let req = pals::cookie::parse(message);
        println!("Verified: From {}: {}.", req["from"], req["tx_list"]);
    } else {
        println!("Invalid signature!");
    }
}

fn part2() {
    let key = pals::aes::random_key();
    let message = "from=unsuspectin&tx_list=hodor:1M";
    let mac = cbc_mac_2(message.as_bytes(), &key);

    verify_request_2(&key, message, &mac);
}

fn main() {
    println!("Part 1");
    println!("------");
    part1();

    println!("");

    println!("Part 2");
    println!("------");
    part2();
}
