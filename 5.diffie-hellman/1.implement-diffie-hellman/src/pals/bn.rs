use std::cmp::{min, max, Ordering, Eq, Ord, PartialEq, PartialOrd};

fn load32(digits: &[u64], index: usize) -> u64 {
    if index % 2 == 0 {
        digits[index / 2] & 0xffffffff
    } else {
        digits[index / 2] >> 32
    }
}

fn store32(digits: &mut [u64], index: usize, value: u64) {
    if index % 2 == 0 {
        digits[index / 2] = (digits[index / 2] & 0xffffffff00000000) | value;
    } else {
        digits[index / 2] = (digits[index / 2] & 0x00000000ffffffff) | (value << 32);
    }
}

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

    pub fn mul(&mut self, rhs: &Self) {
        let mut out_digits = vec![0; self.len() + rhs.len() + 1];

        {
            let lhs_digits = &self.digits;
            let rhs_digits = &rhs.digits;

            for lhs_index in 0..2 * lhs_digits.len() {
                let lhs_value = load32(lhs_digits, lhs_index);
                let mut carry = 0;

                for rhs_index in 0..2 * rhs_digits.len() {
                    let rhs_value = load32(rhs_digits, rhs_index);
                    let out_value = load32(&out_digits, lhs_index + rhs_index);

                    let product = lhs_value * rhs_value + out_value + carry;

                    store32(&mut out_digits, lhs_index + rhs_index, product & 0xffffffff);
                    carry = product >> 32;
                }

                store32(&mut out_digits, lhs_index + 2 * rhs_digits.len(), carry);
            }
        }

        self.digits = out_digits;
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

    #[test]
    fn mul() {
        let mut a = BigNum::from_bytes(&decode("02").unwrap());
        let b = BigNum::from_bytes(&decode("03").unwrap());

        a.mul(&b);
        assert_eq!(encode(&a.as_bytes()), "06");
    }

    #[test]
    fn mul_big() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());

        a.mul(&b);
        assert_eq!(encode(&a.as_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn mul_big2() {
        let mut a = BigNum::from_bytes(&decode("ff01020304050607").unwrap());
        let b = BigNum::from_bytes(&decode("ff01020304050607").unwrap());

        for _ in 0..999 {
            a.add(&b);
        }

        let mut c = BigNum::from_bytes(&decode("ff01020304050607").unwrap());
        let d = BigNum::from_bytes(&decode("03e8").unwrap());

        c.mul(&d);
        assert_eq!(encode(&a.as_bytes()), encode(&c.as_bytes()));
    }

    #[test]
    fn mul_big3() {
        let mut a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());

        a.mul(&b);
        assert_eq!(encode(&a.as_bytes()), "fffffffffffffffe0000000000000001");
    }
}
