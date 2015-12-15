fn bit_count(byte: u8) -> u32 {
    let mut value = byte;
    let mut count = 0;

    while value > 0 {
        count += (value % 2) as u32;
        value /= 2;
    }

    count
}

pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    if a.len() != b.len() {
        return panic!("lengths do not match");
    }

    a.iter().zip(b.iter()).fold(0, |sum, (x, y)| sum + bit_count(x ^ y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_test() {
        assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
    }

    #[test]
    fn hamming_equal_test() {
        assert_eq!(hamming_distance(b"foo", b"foo"), 0);
    }
}
