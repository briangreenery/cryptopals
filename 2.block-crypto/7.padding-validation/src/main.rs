#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn is_valid_padding(block: &[u8]) -> bool {
    if block.len() == 0 {
        return false;
    }

    let amount = block[block.len() - 1];

    for i in 0..amount {
        if block[block.len() - (i as usize) - 1] != amount {
            return false;
        }
    }

    return true;
}

#[test]
fn is_valid_padding_test() {
    assert!(is_valid_padding(b"ICE ICE BABY\x04\x04\x04\x04"));
    assert!(!is_valid_padding(b"ICE ICE BABY\x05\x05\x05\x05"));
    assert!(!is_valid_padding(b"ICE ICE BABY\x01\x02\x03\x04"));
}

fn main() {
    println!("Hello, world!");
}
