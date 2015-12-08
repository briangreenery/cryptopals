fn num_from_hex(hex: u8) -> u8 {
    match hex {
        48...57 => hex - 48,
        65...70 => hex - 55,
        97...102 => hex - 87,
        _ => panic!("invalid hex digit")
    }
}

fn bytes_from_hex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
       .chunks(2)
       .map(|digits: &[u8]| 16 * num_from_hex(digits[0]) + num_from_hex(digits[1]))
       .collect()
}

fn decrypt(bytes: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for byte in bytes.iter() {
        result.push(byte ^ key);
    }

    result
}

fn score(letters: &Vec<u8>) -> usize {
    letters.iter().filter(|&letter| {
        match *letter {
            65...90 => true,
            97...122 => true,
            32 => true,
            _ => false
        }
    }).count()
}

fn main() {
    let mut best_score: usize = 0;
    let mut best_byte: u8 = 0;
    
    let cipher = bytes_from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    
    for byte in (0..255) {
        let score = score(&decrypt(&cipher, byte));
        
        if score > best_score {
            best_score = score;
            best_byte = byte;
        }
    }
    
    println!("key: {}", best_byte);
    println!("text: {}", String::from_utf8(decrypt(&cipher, best_byte)).unwrap());
}
