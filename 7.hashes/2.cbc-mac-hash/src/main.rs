#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn cbc_mac(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = [0; 16];
    let encrypted = pals::aes::encrypt_cbc(data, key, &iv);
    encrypted[encrypted.len() - 16..].to_vec()
}

fn main() {
    println!("example");
    println!("-------");

    let key = b"YELLOW SUBMARINE";
    let js = b"alert('MZA who was that?');\n";

    println!("{}", pals::printable(js));
    println!("");
    println!("{}", pals::hex::encode(&cbc_mac(key, js)));

    println!("");
    println!("evil");
    println!("----");

    let evil_js = b"alert('Ayo, the Wu is back!');\n//";
    let evil_mac = cbc_mac(key, evil_js);
    let evil_last_block_start = 16 * (evil_js.len() / 16);

    let mut crafted = Vec::new();
    crafted.extend(&evil_js[0..evil_last_block_start]);
    crafted.extend(&pals::aes::pad(&evil_js[evil_last_block_start..], 16));

    let mut evil_iv = [0; 16];
    pals::aes::xor(&evil_mac, &mut evil_iv);
    pals::aes::xor(&js[0..16], &mut evil_iv);

    crafted.extend(&evil_iv);
    crafted.extend(&js[16..]);

    println!("{}", pals::printable(&crafted));
    println!("");
    println!("{}", pals::hex::encode(&cbc_mac(key, &crafted)));
}
