fn encode_nibble(num: u8) -> u8 {
    match num {
        0...9 => num + b'0',
        _ => num - 10 + b'a',
    }
}

pub fn encode(bytes: &[u8]) -> String {
    let mut result = Vec::new();

    for byte in bytes.iter() {
        result.push(encode_nibble(byte / 16));
        result.push(encode_nibble(byte % 16));
    }

    String::from_utf8(result).unwrap()
}

#[derive(Debug)]
pub enum Error {
    InvalidChar(u8),
    InvalidLength(usize),
}

fn decode_nibble(hex: u8) -> Result<u8, Error> {
    match hex {
        b'0'...b'9' => Ok(hex - b'0'),
        b'A'...b'F' => Ok(hex - b'A' + 10),
        b'a'...b'f' => Ok(hex - b'a' + 10),
        _ => Err(Error::InvalidChar(hex)),
    }
}

pub fn decode(hex: &str) -> Result<Vec<u8>, Error> {
    let bytes = hex.as_bytes();

    if bytes.len() % 2 != 0 {
        return Err(Error::InvalidLength(bytes.len()));
    }

    let mut result = Vec::new();

    for chunk in bytes.chunks(2) {
        let mut value = 0;

        match decode_nibble(chunk[0]) {
            Ok(n) => value += 16 * n,
            Err(err) => return Err(err),
        }

        match decode_nibble(chunk[1]) {
            Ok(n) => value += n,
            Err(err) => return Err(err),
        }

        result.push(value);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty_test() {
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn encode_cryptopals_test() {
        assert_eq!(encode(b"cryptopals"), "63727970746f70616c73");
    }

    #[test]
    fn decode_empty_test() {
        assert_eq!(decode("").unwrap(), b"");
    }

    #[test]
    fn decode_cryptopals_test() {
        assert_eq!(decode("63727970746f70616c73").unwrap(), b"cryptopals");
        assert_eq!(decode("63727970746F70616C73").unwrap(), b"cryptopals");
    }

    #[test]
    #[should_panic]
    fn decode_bad_length_test() {
        decode("1").unwrap();
    }

    #[test]
    #[should_panic]
    fn decode_bad_char_test() {
        decode("ag").unwrap();
    }
}
