#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn is_valid(data: &[u8]) -> bool {
    for byte in data {
        if *byte > 127 {
            return false;
        }
    }

    return true;
}

fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let decrypted = pals::aes::decrypt_cbc(data, key, key);

    if !is_valid(&decrypted) {
        return Err(decrypted);
    }

    Ok(decrypted)
}

fn main() {
    let key = pals::aes::random_key();
    println!("random key: {}", pals::hex::encode(&key));

    let plain = b"There are two cats who are pretty cute.";
    let cipher = pals::aes::encrypt_cbc(plain, &key, &key);

    let mut evil = Vec::new();
    evil.extend(&cipher[..16]);
    evil.extend(&[0; 16]);
    evil.extend(&cipher[..16]);
    evil.extend(&cipher[cipher.len() - 32..]);

    let decrypted = decrypt(&evil, &key);

    match decrypted {
        Ok(data) => {
            println!("decrypted ok: {}", String::from_utf8(data).unwrap());
        }
        Err(data) => {
            let mut extracted = [0; 16];

            for i in 0..16 {
                extracted[i] = data[i] ^ data[32 + i];
            }

            println!("extracted key: {}", pals::hex::encode(&extracted));
        }
    }
}
