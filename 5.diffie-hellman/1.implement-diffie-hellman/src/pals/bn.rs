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

fn add_digit(out: &mut [u32], i: usize, num: u32) -> u32 {
    let overflow = out[i] > u32::max_value() - num;
    out[i] = out[i].wrapping_add(num);

    if overflow {
        1
    } else {
        0
    }
}

fn add(lhs: &mut [u32], rhs: &[u32]) {
    let mut carry = 0;
    let mut index = 0;

    while index < rhs.len() {
        carry = add_digit(lhs, index, carry) + add_digit(lhs, index, rhs[index]);
        index += 1;
    }

    while index < lhs.len() {
        carry = add_digit(lhs, index, carry);
        index += 1;
    }
}

fn sub_digit(out: &mut [u32], i: usize, num: u32) -> u32 {
    let overflow = num > out[i];
    out[i] = out[i].wrapping_sub(num);

    if overflow {
        1
    } else {
        0
    }
}

fn sub(lhs: &mut [u32], rhs: &[u32]) -> u32 {
    let mut carry = 0;
    let mut index = 0;

    while index < rhs.len() {
        carry = sub_digit(lhs, index, carry) + sub_digit(lhs, index, rhs[index]);
        index += 1;
    }

    while index < lhs.len() {
        carry = sub_digit(lhs, index, carry);
        index += 1;
    }

    carry
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

fn div_by_one_digit(lhs: &[u32], rhs: u32, base: u64) -> (Vec<u32>, u32) {
    let mut quotient = vec![0; lhs.len()];

    let mut remainder = lhs.to_vec();
    remainder.push(0);

    for i in (0..remainder.len() - 1).rev() {
        let lhs_digit = ((remainder[i + 1] as u64) * base) + (remainder[i] as u64);
        let rhs_digit = rhs as u64;

        quotient[i] = (lhs_digit / rhs_digit) as u32;
        remainder[i] = (lhs_digit % rhs_digit) as u32;
    }

    truncate_zeroes(&mut quotient);
    (quotient, remainder[0])
}

fn base_100000_from_str(decimal: &str) -> Vec<u32> {
    let chars = decimal.as_bytes();

    let mut digits = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        let mut digit = 0;

        let start = chars.len() - min(index + 5, chars.len());
        let end = chars.len() - index;

        for i in start..end {
            digit *= 10;
            digit += (chars[i] - b'0') as u32;
        }

        digits.push(digit);
        index += 5;
    }

    truncate_zeroes(&mut digits);
    digits
}

pub struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    pub fn new() -> Self {
        BigNum { digits: Vec::new() }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut digits = Vec::new();
        let mut index = 0;

        while index < bytes.len() {
            let mut digit = 0;

            let start = bytes.len() - min(index + 4, bytes.len());
            let end = bytes.len() - index;

            for i in start..end {
                digit <<= 8;
                digit += bytes[i] as u32;
            }

            digits.push(digit);
            index += 4;
        }

        truncate_zeroes(&mut digits);
        BigNum { digits: digits }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for digit in self.digits.iter() {
            for i in 0..4 {
                bytes.push(((*digit >> (8 * i)) & 0xff) as u8);
            }
        }

        truncate_zeroes(&mut bytes);
        if bytes.len() == 0 {
            bytes.push(0);
        }

        bytes.reverse();
        bytes
    }

    pub fn from_decimal(decimal: &str) -> Self {
        let mut bytes = Vec::new();
        let mut value = base_100000_from_str(decimal);

        while value.len() > 0 {
            let result = div_by_one_digit(&value, 256, 100000);
            bytes.push(result.1 as u8);
            value = result.0;
        }

        bytes.reverse();
        Self::from_bytes(&bytes)
    }

    pub fn to_decimal(&self) -> String {
        let mut decimal = Vec::new();
        let mut value = self.digits.clone();

        while value.len() > 0 {
            let result = div_by_one_digit(&value, 10, 0x100000000);
            decimal.push(b'0' + (result.1 as u8));
            value = result.0;
        }

        if decimal.len() == 0 {
            decimal.push(b'0');
        }

        decimal.reverse();
        String::from_utf8(decimal).unwrap()
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let mut result = self.digits.clone();

        result.push(0);
        while result.len() < rhs.digits.len() + 1 {
            result.push(0);
        }

        add(&mut result, &rhs.digits);

        truncate_zeroes(&mut result);
        BigNum { digits: result }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        let mut result = self.digits.clone();

        if result.len() < rhs.digits.len() || sub(&mut result, &rhs.digits) != 0 {
            panic!("cannot subtract larger value");
        }

        truncate_zeroes(&mut result);
        BigNum { digits: result }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut result = mul(&self.digits, &rhs.digits);

        truncate_zeroes(&mut result);
        BigNum { digits: result }
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
        assert_eq!(encode(&a.to_bytes()), "01");
    }

    #[test]
    fn hex_large() {
        let a = BigNum::from_bytes(&decode("0100000000000000000000").unwrap());
        assert_eq!(encode(&a.to_bytes()), "0100000000000000000000");
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
        assert_eq!(encode(&b.to_bytes()), "02");
    }

    #[test]
    fn add_with_carry1() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.add(&b);
        assert_eq!(encode(&c.to_bytes()), "010000000000000000");
    }

    #[test]
    fn add_with_carry2() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = a.add(&a);
        assert_eq!(encode(&b.to_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn add_with_carry3() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = a.add(&a);
        assert_eq!(encode(&b.to_bytes()), "01fffffffffffffffffffffffffffffffe");
    }

    #[test]
    fn add_with_carry4() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffffffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.add(&b);
        assert_eq!(encode(&c.to_bytes()), "0100000000000000000000000000000000");
    }

    #[test]
    fn sub() {
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = a.sub(&a);
        assert_eq!(encode(&b.to_bytes()), "00");
    }

    #[test]
    fn sub_with_carry() {
        let a = BigNum::from_bytes(&decode("0100000000000000000000000000000000").unwrap());
        let b = BigNum::from_bytes(&decode("01").unwrap());
        let c = a.sub(&b);
        assert_eq!(encode(&c.to_bytes()), "ffffffffffffffffffffffffffffffff");
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
        assert_eq!(encode(&c.to_bytes()), "06");
    }

    #[test]
    fn mul_big() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("02").unwrap());
        let c = a.mul(&b);
        assert_eq!(encode(&c.to_bytes()), "01fffffffffffffffe");
    }

    #[test]
    fn mul_big2() {
        let a = BigNum::from_bytes(&decode("ff01020304050607").unwrap());
        let b = BigNum::from_bytes(&decode("03e8").unwrap());
        let c = a.mul(&b);
        assert_eq!(encode(&c.to_bytes()), "03e41befdbc7b39f8b58");
    }

    #[test]
    fn mul_big3() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = a.mul(&a);
        assert_eq!(encode(&b.to_bytes()), "fffffffffffffffe0000000000000001");
    }

    #[test]
    fn to_decimal0() {
        let a = BigNum::from_bytes(&decode("00").unwrap());
        assert_eq!(a.to_decimal(), "0");
    }

    #[test]
    fn to_decimal1() {
        let a = BigNum::from_bytes(&decode("0a").unwrap());
        assert_eq!(a.to_decimal(), "10");
    }

    #[test]
    fn to_decimal2() {
        let a = BigNum::from_bytes(&decode("bc614e").unwrap());
        assert_eq!(a.to_decimal(), "12345678");
    }

    #[test]
    fn to_decimal3() {
        let a = BigNum::from_bytes(&decode("bc614e").unwrap());
        let b = a.mul(&a).mul(&a).mul(&a).mul(&a);
        assert_eq!(b.to_decimal(), "286797081492411793084216657371142368");
    }

    #[test]
    fn from_decimal() {
        let number = "1238716238761387364853498121837294658376482763428319";
        let a = BigNum::from_decimal(number);
        assert_eq!(a.to_decimal(), number);
    }
}
