extern crate crypto;
extern crate rand;

use std;
use crypto::symmetriccipher::{Encryptor, Decryptor};
use crypto::buffer::{ReadBuffer, WriteBuffer};
use rand::Rng;

pub fn encrypt_block(block: &[u8], key: &[u8], output: &mut Vec<u8>) {
    let mut encryptor = crypto::aes::ecb_encryptor(crypto::aes::KeySize::KeySize128,
                                                   key,
                                                   crypto::blockmodes::NoPadding);

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&block);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }
}

pub fn decrypt_block(block: &[u8], key: &[u8], output: &mut Vec<u8>) {
    let mut decryptor = crypto::aes::ecb_decryptor(crypto::aes::KeySize::KeySize128,
                                                   key,
                                                   crypto::blockmodes::NoPadding);

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&block);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        output.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }
}

pub fn pad(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let mut result = bytes.to_vec();
    let amount = (block_size - bytes.len()) as u8;

    while result.len() < block_size {
        result.push(amount);
    }

    result
}

pub fn is_valid_padding(block: &[u8]) -> bool {
    if block.len() == 0 {
        return false;
    }

    let amount = block[block.len() - 1];

    if amount == 0 {
        return false;
    }

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
    assert!(!is_valid_padding(b"ICE ICE BABY\x01\x02\x03\x00"));
}

pub fn encrypt_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for block in data.chunks(16) {
        let padded = if block.len() < 16 {
            pad(block, 16)
        } else {
            block.to_vec()
        };

        encrypt_block(&padded, key, &mut result);
    }

    if data.len() % 16 == 0 {
        encrypt_block(&[16; 16], key, &mut result);
    }

    result
}

pub fn decrypt_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for block in data.chunks(16) {
        decrypt_block(block, key, &mut result);
    }

    let size_without_padding = result.len() - (result[result.len() - 1] as usize);
    result.truncate(size_without_padding);

    result
}

pub fn xor(last: &[u8], current: &mut [u8]) {
    let min = std::cmp::min(last.len(), current.len());
    for i in 0..min {
        current[i] = current[i] ^ last[i];
    }
}

pub fn encrypt_cbc(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut last = iv.to_vec();

    for block in data.chunks(16) {
        let mut padded = if block.len() < 16 {
            pad(block, 16)
        } else {
            block.to_vec()
        };

        xor(&last, &mut padded);
        encrypt_block(&padded, key, &mut result);
        last = result[result.len() - 16..result.len()].to_vec();
    }

    if data.len() % 16 == 0 {
        let mut padded = [16; 16];
        xor(&last, &mut padded);
        encrypt_block(&padded, key, &mut result);
    }

    result
}

pub fn decrypt_cbc(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut last = iv.to_vec();

    for block in data.chunks(16) {
        decrypt_block(block, key, &mut result);

        let start = result.len() - 16;
        let end = result.len();

        xor(&last, &mut result[start..end]);
        last = block.to_vec();
    }

    let size_without_padding = result.len() - (result[result.len() - 1] as usize);
    result.truncate(size_without_padding);

    result
}

pub fn random_key() -> Vec<u8> {
    let mut key = [0; 16];
    rand::thread_rng().fill_bytes(&mut key);
    key.to_vec()
}

pub struct CTR {
    key: Vec<u8>,
    nonce: i64,
}

impl CTR {
    pub fn new(key: &[u8], nonce: i64) -> CTR {
        CTR {
            key: key.to_vec(),
            nonce: nonce,
        }
    }

    fn block(&self, count: i64) -> Vec<u8> {
        let mut block: [u8; 16] = [0; 16];

        for i in 0..8 {
            block[i] = ((self.nonce >> (8 * i)) & 0xff) as u8;
        }

        for i in 0..8 {
            block[8 + i] = ((count >> (8 * i)) & 0xff) as u8;
        }

        let mut result = Vec::new();
        encrypt_block(&block, &self.key, &mut result);
        result
    }

    pub fn apply(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();

        for (count, block) in data.chunks(16).enumerate() {
            let mut next = self.block(count as i64);
            xor(block, &mut next);
            result.extend(&next[0..block.len()]);
        }

        result
    }

    pub fn edit(&self, old: &[u8], offset: usize, new: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();

        if offset != 0 {
            result.extend(&old[..offset]);
        }

        if result.len() % 16 != 0 {
            let mut block = self.block((result.len() / 16) as i64);

            let block_start = result.len() % 16;
            let len = std::cmp::min(new.len(), 16 - block_start);

            xor(&new[..len], &mut block[block_start..]);
            result.extend(&block[block_start..block_start + len]);
        }

        while result.len() != offset + new.len() {
            let mut block = self.block((result.len() / 16) as i64);

            let new_start = result.len() - offset;
            let len = std::cmp::min(16, new.len() - new_start);

            xor(&new[new_start..new_start + len], &mut block[..len]);
            result.extend(&block[..len]);
        }

        if offset + new.len() < old.len() {
            result.extend(&old[offset + new.len()..]);
        }

        result
    }
}
