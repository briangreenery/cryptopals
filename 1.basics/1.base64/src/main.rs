fn hex_digit_value(digit: u8) -> u8 {
    match digit {
        48...57 => digit - 48,
        65...90 => digit - 65 + 10,
        97...122 => digit - 97 + 10,
        _ => panic!("invalid hex digit")
    }
}

fn bytes_from_hex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
       .chunks(2)
       .map(|digits: &[u8]| 16 * hex_digit_value(digits[0]) + hex_digit_value(digits[1]))
       .collect()
}

fn base64_value(accum: u32, index: u32) -> u8 {
    let value = ((accum >> (18 - 6 * index)) % 64) as u8; 
    
    match value {
        0...25 => value + 65,
        26...51 => value + 71,
        52...61 => value - 4,
        62 => 43,
        _ => 47
    }
}

fn base64_from_bytes(bytes: &Vec<u8>) -> String {
    let mut result: Vec<u8> = Vec::new();
    let mut accum: u32 = 0;

    for (index, byte) in bytes.iter().enumerate() {
        accum <<= 8;
        accum |= *byte as u32;

        if (index + 1) % 3 == 0 {
            result.push(base64_value(accum, 0));
            result.push(base64_value(accum, 1));
            result.push(base64_value(accum, 2));
            result.push(base64_value(accum, 3));
            accum = 0;
        }
    }

    match bytes.len() % 3 {
        1 => {
            accum <<= 16;
            result.push(base64_value(accum, 0));
            result.push(base64_value(accum, 1));
            result.push(61);
            result.push(61);
        }
        2 => {
            accum <<= 8;
            result.push(base64_value(accum, 0));
            result.push(base64_value(accum, 1));
            result.push(base64_value(accum, 2));
            result.push(61);
        }
        _ => {}
    }

    return String::from_utf8(result).unwrap();
}

fn base64_from_hex(hex: &str) -> String {
    base64_from_bytes(&bytes_from_hex(hex))
}

fn main() {
    println!("{}",
             base64_from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69\
                              736f6e6f7573206d757368726f6f6d"));
}

#[test]
fn empty_test() {
    assert_eq!(base64_from_hex(""), "");
}

#[test]
fn padding_one_test() {
    assert_eq!(base64_from_hex("61"), "YQ==");
}

#[test]
fn padding_two_test() {
    assert_eq!(base64_from_hex("6162"), "YWI=");
}

#[test]
fn padding_three_test() {
    assert_eq!(base64_from_hex("616263"), "YWJj");
}

#[test]
fn example_test() {
    assert_eq!(base64_from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f\
                                69736f6e6f7573206d757368726f6f6d"),
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
