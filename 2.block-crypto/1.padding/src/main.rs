#![allow(dead_code)]
mod pals;

fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let mut result = bytes.to_vec();
    let amount = (block_size - bytes.len()) as u8;

    while result.len() < block_size {
        result.push(amount);
    }

    result
}

fn main() {
    println!("{}", pals::hex::encode(&pad(b"YELLOW SUBMARINE", 20)));
}
