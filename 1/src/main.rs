fn hex_digit_value(digit: u8) -> u8 {
    match digit {
        48...57 => digit - 48,
        65...90 => digit - 65 + 10,
        97...122 => digit - 97 + 10,
        _ => panic!("invalid hex digit"),
    }
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
       .chunks(2)
       .map(|digits: &[u8]| 16 * hex_digit_value(digits[0]) + hex_digit_value(digits[1]))
       .collect()
}

fn base64_value(accum: u32, index: u32) -> usize {
    ((accum >> (18 - 6 * index)) % 64) as usize
}

fn bytes_to_base64(bytes: &Vec<u8>) -> String {
    let table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

    let mut result: Vec<u8> = Vec::new();
    let mut accum: u32 = 0;
    let mut count: u8 = 0;

    for byte in bytes.iter() {
        accum <<= 8;
        accum |= *byte as u32;

        count += 1;
        if count == 3 {
            for i in 0..4 {
                result.push(table[base64_value(accum, i)]);
            }

            accum = 0;
            count = 0;
        }
    }

    match count {
        1 => {
            accum <<= 16;
            result.push(table[base64_value(accum, 0)]);
            result.push(table[base64_value(accum, 1)]);
            result.push(61);
            result.push(61);
        }
        2 => {
            accum <<= 8;
            result.push(table[base64_value(accum, 0)]);
            result.push(table[base64_value(accum, 1)]);
            result.push(table[base64_value(accum, 2)]);
            result.push(61);
        }
        _ => {}
    }

    return String::from_utf8(result).unwrap();
}

fn hex_to_base64(hex: &str) -> String {
    bytes_to_base64(&hex_to_bytes(hex))
}

fn main() {
    println!("{}",
             hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f6973\
                            6f6e6f7573206d757368726f6f6d"));
}

#[test]
fn empty_test() {
    assert_eq!(hex_to_base64(""), "");
}

#[test]
fn padding_one_test() {
    assert_eq!(hex_to_base64("61"), "YQ==");
}

#[test]
fn padding_two_test() {
    assert_eq!(hex_to_base64("6162"), "YWI=");
}

#[test]
fn padding_three_test() {
    assert_eq!(hex_to_base64("616263"), "YWJj");
}

#[test]
fn example_test() {
    assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69\
                              736f6e6f7573206d757368726f6f6d"),
               "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}
