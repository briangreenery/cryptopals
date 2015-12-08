fn num_from_hex(hex: u8) -> u8 {
    match hex {
        48...57 => hex - 48,
        65...70 => hex - 55,
        97...102 => hex - 87,
        _ => panic!("invalid hex digit")
    }
}

fn hex_from_num(num: u8) -> u8 {
    match num {
        0...9 => num + 48,
        _ => num + 87,
    }
}

fn bytes_from_hex(hex: &str) -> Vec<u8> {
    hex.as_bytes()
       .chunks(2)
       .map(|digits: &[u8]| 16 * num_from_hex(digits[0]) + num_from_hex(digits[1]))
       .collect()
}

fn hex_from_bytes(bytes: &Vec<u8>) -> String {
    let mut result: Vec<u8> = Vec::new();

    for byte in bytes.iter() {
        result.push(hex_from_num(byte / 16));
        result.push(hex_from_num(byte % 16));
    }

    String::from_utf8(result).unwrap()
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for (x, y) in a.iter().zip(b) {
        result.push(x ^ y);
    }

    result
}

fn main() {
    let x = "1c0111001f010100061a024b53535009181c";
    let y = "686974207468652062756c6c277320657965";
    
    let result = xor(&bytes_from_hex(x), &bytes_from_hex(y));
    println!("{} xor {} = {}", x, y, hex_from_bytes(&result));
}

#[test]
fn example_test() {
    assert_eq!(hex_from_bytes(&xor(&bytes_from_hex("1c0111001f010100061a024b53535009181c"),
                                   &bytes_from_hex("686974207468652062756c6c277320657965"))),
               "746865206b696420646f6e277420706c6179");
}
