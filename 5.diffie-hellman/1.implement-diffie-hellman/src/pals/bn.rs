use std::cmp::min;

pub struct BigNum {
    digits: Vec<u64>,
}

impl BigNum {
    pub fn new() -> BigNum {
        BigNum { digits: Vec::new() }
    }

    pub fn from_bytes(bytes: &[u8]) -> BigNum {
        let mut digits = Vec::new();
        let mut index = 0;

        while index < bytes.len() {
            let mut digit = 0;

            let start = bytes.len() - min(index + 8, bytes.len());
            let end = bytes.len() - index;

            for i in start..end {
                digit *= 256;
                digit += bytes[i] as u64;
            }

            digits.push(digit);
            index += 8;
        }

        BigNum { digits: digits }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for digit in self.digits.iter() {
            let mut value = *digit;

            for _ in 0..8 {
                bytes.push((value % 256) as u8);
                value /= 256;
            }
        }

        while *bytes.last().unwrap_or(&1) == 0 {
            bytes.pop();
        }

        bytes.reverse();
        bytes
    }

    fn ensure_digit_at(&mut self, index: usize) {
        while self.digits.len() <= index {
            self.digits.push(0);
        }
    }

    fn add_digit(&mut self, index: usize, amount: u64) -> u64 {
        self.ensure_digit_at(index);

        let overflow = self.digits[index] > u64::max_value() - amount;
        self.digits[index] = self.digits[index].wrapping_add(amount);

        if overflow {
            1
        } else {
            0
        }
    }

    pub fn add(&mut self, amount: &BigNum) {
        let mut carry = 0;
        let mut index = 0;

        for &digit in amount.digits.iter() {
            carry = self.add_digit(index, carry) + self.add_digit(index, digit);
            index += 1;
        }

        while carry != 0 {
            carry = self.add_digit(index, carry);
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hex::*;

    #[test]
    fn hex_small() {
        let a = BigNum::from_bytes(&decode("0000000000000000000001").unwrap());
        assert_eq!(encode(&a.as_bytes()), "01");
    }

    #[test]
    fn hex_large() {
        let a = BigNum::from_bytes(&decode("0100000000000000000000").unwrap());
        assert_eq!(encode(&a.as_bytes()), "0100000000000000000000");
    }

    #[test]
    fn add() {
        let mut a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());

        a.add(&b);
        assert_eq!(encode(&a.as_bytes()), "02");
    }

    #[test]
    fn add_with_carry1() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());

        a.add(&b);
        assert_eq!(encode(&a.as_bytes()), "010000000000000000");
    }

    #[test]
    fn add_with_carry2() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());

        a.add(&b);
        assert_eq!(encode(&a.as_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn add_with_carry3() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());

        a.add(&b);
        assert_eq!(encode(&a.as_bytes()), "01fffffffffffffffffffffffffffffffe");
    }

    #[test]
    fn add_with_carry4() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());

        a.add(&b);
        assert_eq!(encode(&a.as_bytes()), "0100000000000000000000000000000000");
    }
}
