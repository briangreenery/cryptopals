use std::cmp::{min, max, Ordering, Eq, Ord, PartialEq, PartialOrd};

fn truncate_zeroes<T: Eq + From<u8>>(digits: &mut Vec<T>) {
    let mut count = digits.len();

    for digit in digits.iter().rev() {
        if *digit != T::from(0) {
            break;
        }

        count -= 1;
    }

    digits.truncate(count);
}

fn cmp(lhs: &[u32], rhs: &[u32]) -> Ordering {
    let max_len = max(lhs.len(), rhs.len());

    for i in (0..max_len).rev() {
        let lhs_digit = *lhs.get(i).unwrap_or(&0);
        let rhs_digit = *rhs.get(i).unwrap_or(&0);

        let order = lhs_digit.cmp(&rhs_digit);

        if order != Ordering::Equal {
            return order;
        }
    }

    Ordering::Equal
}

fn add_digit(out: &mut Vec<u32>, i: usize, num: u32) -> u32 {
    let overflow = out[i] > u32::max_value() - num;
    out[i] = out[i].wrapping_add(num);

    if overflow {
        1
    } else {
        0
    }
}

fn add(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
    let mut out = vec![0; max(lhs.len(), rhs.len()) + 1];
    let mut carry = 0;

    for i in 0..out.len() {
        carry = add_digit(&mut out, i, carry);

        if i < lhs.len() {
            carry += add_digit(&mut out, i, lhs[i])
        }

        if i < rhs.len() {
            carry += add_digit(&mut out, i, rhs[i]);
        }
    }

    truncate_zeroes(&mut out);
    out
}

fn sub_digit(out: &mut Vec<u32>, i: usize, num: u32) -> u32 {
    let overflow = num > out[i];
    out[i] = out[i].wrapping_sub(num);

    if overflow {
        1
    } else {
        0
    }
}

fn sub(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
    if cmp(lhs, rhs) == Ordering::Less {
        panic!("cannot subtract larger value");
    }

    let mut out = lhs.to_vec();
    let mut carry = 0;

    for i in 0..out.len() {
        carry = sub_digit(&mut out, i, carry);

        if i < rhs.len() {
            carry += sub_digit(&mut out, i, rhs[i]);
        }
    }

    out
}

fn mul(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
    let mut out = vec![0; lhs.len() + rhs.len() + 1];

    for (lhs_index, lhs_digit) in lhs.iter().enumerate() {
        let lhs_value = *lhs_digit as u64;
        let mut carry = 0;

        for (rhs_index, rhs_digit) in rhs.iter().enumerate() {
            let rhs_value = *rhs_digit as u64;
            let out_value = out[lhs_index + rhs_index] as u64;

            let product = lhs_value * rhs_value + out_value + carry;

            out[lhs_index + rhs_index] = (product & 0xffffffff) as u32;
            carry = product >> 32;
        }

        out[lhs_index + rhs.len()] = carry as u32;
    }

    out
}

pub struct BigNum {
    digits: Vec<u32>,
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

            let start = bytes.len() - min(index + 4, bytes.len());
            let end = bytes.len() - index;

            for i in start..end {
                digit *= 256;
                digit += bytes[i] as u32;
            }

            digits.push(digit);
            index += 4;
        }

        BigNum { digits: digits }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for digit in self.digits.iter() {
            let mut value = *digit;

            for _ in 0..4 {
                bytes.push((value % 256) as u8);
                value /= 256;
            }
        }

        truncate_zeroes(&mut bytes);

        if bytes.len() == 0 {
            bytes.push(0);
        }

        bytes.reverse();
        bytes
    }

    pub fn add(&self, rhs: &Self) -> Self {
        BigNum { digits: add(&self.digits, &rhs.digits) }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        BigNum { digits: sub(&self.digits, &rhs.digits) }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        BigNum { digits: mul(&self.digits, &rhs.digits) }
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(&self.digits, &other.digits)
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
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = a.add(&a);
        assert_eq!(encode(&b.as_bytes()), "02");
    }

    #[test]
    fn add_with_carry1() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.add(&b);
        assert_eq!(encode(&c.as_bytes()), "010000000000000000");
    }

    #[test]
    fn add_with_carry2() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = a.add(&a);
        assert_eq!(encode(&b.as_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn add_with_carry3() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = a.add(&a);
        assert_eq!(encode(&b.as_bytes()), "01fffffffffffffffffffffffffffffffe");
    }

    #[test]
    fn add_with_carry4() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.add(&b);
        assert_eq!(encode(&c.as_bytes()), "0100000000000000000000000000000000");
    }

    #[test]
    fn sub() {
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = a.sub(&a);
        assert_eq!(encode(&b.as_bytes()), "00");
    }

    #[test]
    fn sub_with_carry() {
        let a = BigNum::from_bytes(&decode("0100000000000000000000000000000000").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.sub(&b);
        assert_eq!(encode(&c.as_bytes()), "ffffffffffffffffffffffffffffffff");
    }

    #[test]
    #[should_panic]
    fn sub_negative() {
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());
        a.sub(&b);
    }

    #[test]
    fn mul() {
        let a = BigNum::from_bytes(&decode("02").unwrap());
        let b = BigNum::from_bytes(&decode("03").unwrap());
        let c = a.mul(&b);
        assert_eq!(encode(&c.as_bytes()), "06");
    }

    #[test]
    fn mul_big() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());
        let c = a.mul(&b);
        assert_eq!(encode(&c.as_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn mul_big2() {
        let a = BigNum::from_bytes(&decode("ff01020304050607").unwrap());
        let b = BigNum::from_bytes(&decode("03e8").unwrap());
        let c = a.mul(&b);
        assert_eq!(encode(&c.as_bytes()), "03e41befdbc7b39f8b58");
    }

    #[test]
    fn mul_big3() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = a.mul(&a);
        assert_eq!(encode(&b.as_bytes()), "fffffffffffffffe0000000000000001");
    }
}
