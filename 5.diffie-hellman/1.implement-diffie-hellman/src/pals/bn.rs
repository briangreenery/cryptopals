use std::cmp::{min, max, Ordering, Eq, Ord, PartialEq, PartialOrd};

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

        if bytes.len() == 0 {
            bytes.push(0);
        }

        bytes.reverse();
        bytes
    }

    fn digit(&self, index: usize) -> u64 {
        *self.digits.get(index).unwrap_or(&0)
    }

    fn len(&self) -> usize {
        self.digits.len()
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

    pub fn add(&mut self, rhs: &Self) {
        let mut carry = 0;
        let mut index = 0;

        for &digit in rhs.digits.iter() {
            carry = self.add_digit(index, carry) + self.add_digit(index, digit);
            index += 1;
        }

        while carry != 0 {
            carry = self.add_digit(index, carry);
            index += 1;
        }
    }

    fn can_sub(&self, rhs: &Self) -> bool {
        self >= rhs
    }

    fn sub_digit(&mut self, index: usize, amount: u64) -> u64 {
        let overflow = amount > self.digits[index];
        self.digits[index] = self.digits[index].wrapping_sub(amount);

        if overflow {
            1
        } else {
            0
        }
    }

    pub fn sub(&mut self, rhs: &Self) {
        if !self.can_sub(rhs) {
            panic!("cannot subtract larger value");
        }

        let mut carry = 0;
        let mut index = 0;

        for &digit in rhs.digits.iter() {
            carry = self.sub_digit(index, carry) + self.sub_digit(index, digit);
            index += 1;
        }

        while carry != 0 {
            carry = self.sub_digit(index, carry);
            index += 1;
        }
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        let max_len = max(self.len(), other.len());

        for n in (0..max_len).rev() {
            let order = self.digit(n).cmp(&other.digit(n));

            if order != Ordering::Equal {
                return order;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for BigNum {}

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
    fn cmp_test() {
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());

        assert!(a == a);
        assert!(a <= a);
        assert!(a >= a);
        assert!(!(a != a));
        assert!(!(a < a));
        assert!(!(a > a));

        assert!(a != b);
        assert!(b != a);
        assert!(a < b);
        assert!(a <= b);
        assert!(b > a);
        assert!(b >= a);

        assert!(!(a == b));
        assert!(!(b == a));
        assert!(!(a > b));
        assert!(!(a >= b));
        assert!(!(b < a));
        assert!(!(b <= a));

        let big = BigNum::from_bytes(&decode("010000000000000000").unwrap());

        assert!(big > b);
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

    #[test]
    fn sub() {
        let mut a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());

        a.sub(&b);
        assert_eq!(encode(&a.as_bytes()), "00");
    }

    #[test]
    fn sub_with_carry() {
        let mut a = BigNum::from_bytes(&decode("0100000000000000000000000000000000").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());

        a.sub(&b);
        assert_eq!(encode(&a.as_bytes()), "ffffffffffffffffffffffffffffffff");
    }

    #[test]
    #[should_panic]
    fn sub_negative() {
        let mut a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());

        a.sub(&b);
    }
}
