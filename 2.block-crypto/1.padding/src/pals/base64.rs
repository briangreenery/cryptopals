fn to_letter(accum: u32, index: u32) -> u8 {
    let value = ((accum >> (18 - 6 * index)) % 64) as u8;

    match value {
        0...25 => value + b'A',
        26...51 => value - 26 + b'a',
        52...61 => value - 52 + b'0',
        62 => b'+',
        _ => b'/',
    }
}

pub fn encode(bytes: &[u8]) -> String {
    let mut result = Vec::new();
    let mut accum = 0;

    for (index, byte) in bytes.iter().enumerate() {
        accum <<= 8;
        accum |= *byte as u32;

        if (index + 1) % 3 == 0 {
            result.push(to_letter(accum, 0));
            result.push(to_letter(accum, 1));
            result.push(to_letter(accum, 2));
            result.push(to_letter(accum, 3));
            accum = 0;
        }
    }

    match bytes.len() % 3 {
        1 => {
            accum <<= 16;
            result.push(to_letter(accum, 0));
            result.push(to_letter(accum, 1));
            result.push(b'=');
            result.push(b'=');
        }
        2 => {
            accum <<= 8;
            result.push(to_letter(accum, 0));
            result.push(to_letter(accum, 1));
            result.push(to_letter(accum, 2));
            result.push(b'=');
        }
        _ => {}
    }

    String::from_utf8(result).unwrap()
}

#[derive(Debug)]
pub enum Error {
    InvalidChar(u8),
    InvalidLength(usize),
}

fn from_letter(letter: u8) -> Result<u8, Error> {
    match letter {
        b'A'...b'Z' => Ok(letter - b'A'),
        b'a'...b'z' => Ok(letter - b'a' + 26),
        b'0'...b'9' => Ok(letter - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(Error::InvalidChar(letter)),
    }
}

pub fn decode(text: &str) -> Result<Vec<u8>, Error> {
    if text.len() % 4 != 0 {
        return Err(Error::InvalidLength(text.len()));
    }

    let mut result = Vec::new();

    for (index, chunk) in text.as_bytes().chunks(4).enumerate() {
        let mut length = chunk.len();
        let mut accum: u32 = 0;

        if index + 1 == text.len() / 4 {
            if chunk[2] == b'=' && chunk[3] == b'=' {
                length = 2;
            } else if chunk[3] == b'=' {
                length = 3;
            }
        }

        for letter in chunk.iter().take(length) {
            match from_letter(*letter) {
                Ok(value) => {
                    accum <<= 6;
                    accum += value as u32;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        accum <<= 6 * (4 - length);

        for i in (0..length - 1) {
            result.push(((accum >> (16 - 8 * i)) % 256) as u8);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_c_test() {
        assert_eq!(encode(b"c"), "Yw==");
    }

    #[test]
    fn encode_ca_test() {
        assert_eq!(encode(b"ca"), "Y2E=");
    }

    #[test]
    fn encode_cat_test() {
        assert_eq!(encode(b"cat"), "Y2F0");
    }

    #[test]
    fn decode_c_test() {
        assert_eq!(decode("Yw==").unwrap(), b"c");
    }

    #[test]
    fn decode_ca_test() {
        assert_eq!(decode("Y2E=").unwrap(), b"ca");
    }

    #[test]
    fn decode_cat_test() {
        assert_eq!(decode("Y2F0").unwrap(), b"cat");
    }

    #[test]
    #[should_panic]
    fn decode_bad_length_test() {
        decode("Y2F").unwrap();
    }

    #[test]
    #[should_panic]
    fn decode_bad_char_test() {
        decode("Y2F=AAAA").unwrap();
    }
}
