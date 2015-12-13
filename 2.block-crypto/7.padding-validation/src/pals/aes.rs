extern crate crypto;

use crypto::symmetriccipher::{Encryptor, Decryptor};
use crypto::buffer::{ReadBuffer, WriteBuffer};

fn encrypt_block(block: &[u8], key: &[u8], output: &mut Vec<u8>) {
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

fn decrypt_block(block: &[u8], key: &[u8], output: &mut Vec<u8>) {
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

fn xor(last: &[u8], current: &mut [u8]) {
    for i in 0..16 {
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
