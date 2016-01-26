const BASE: u64 = 0x100000000;

use std::cmp::{min, max, Ordering, Eq, Ord, PartialEq, PartialOrd};

fn zeropad(digits: &mut Vec<u32>, len: usize) {
    while digits.len() < len {
        digits.push(0);
    }
}

fn trim<T: Eq + From<u8>>(digits: &mut Vec<T>) {
    let mut count = digits.len();

    for digit in digits.iter().rev() {
        if *digit != T::from(0) {
            break;
        }

        count -= 1;
    }

    digits.truncate(count);
}

fn lshift(digits: &mut [u32], amount: u32) {
    for i in (1..digits.len()).rev() {
        digits[i] = (digits[i] << amount) | (digits[i - 1] >> (32 - amount));
    }

    digits[0] <<= amount;
}

fn rshift(digits: &mut [u32], amount: u32) {
    for i in 0..digits.len() - 1 {
        digits[i] = (digits[i] >> amount) | (digits[i + 1] << (32 - amount));
    }

    let len = digits.len();
    digits[len - 1] >>= amount;
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

fn add(lhs: &mut [u32], rhs: &[u32]) {
    let mut carry = 0;

    for i in 0..rhs.len() {
        let sum = (lhs[i] as u64) + (rhs[i] as u64) + carry;
        lhs[i] = (sum & 0xffffffff) as u32;
        carry = sum >> 32;
    }

    for i in rhs.len()..lhs.len() {
        let sum = (lhs[i] as u64) + carry;
        lhs[i] = (sum & 0xffffffff) as u32;
        carry = sum >> 32;
    }
}

fn sub(lhs: &mut [u32], rhs: &[u32]) -> u32 {
    let mut carry = 0;

    for i in 0..rhs.len() {
        let sum = (BASE + (lhs[i] as u64)) - (rhs[i] as u64) - carry;
        lhs[i] = (sum & 0xffffffff) as u32;
        carry = 1 - (sum >> 32);
    }

    for i in rhs.len()..lhs.len() {
        let sum = (BASE + (lhs[i] as u64)) - carry;
        lhs[i] = (sum & 0xffffffff) as u32;
        carry = 1 - (sum >> 32);
    }

    carry as u32
}

fn mul(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
    let mut out = vec![0; lhs.len() + rhs.len()];

    for i in 0..lhs.len() {
        let mut carry = 0;

        for j in 0..rhs.len() {
            let product = (lhs[i] as u64) * (rhs[j] as u64) + (out[i + j] as u64) + carry;
            out[i + j] = (product & 0xffffffff) as u32;
            carry = product >> 32;
        }

        out[i + rhs.len()] = carry as u32;
    }

    out
}

fn div_by_one(lhs: &[u32], rhs: u32) -> (Vec<u32>, Vec<u32>) {
    let mut quotient = vec![0; lhs.len()];
    let mut remainder = 0;

    for i in (0..lhs.len()).rev() {
        let lhs_digit = (remainder << 32) + (lhs[i] as u64);
        let rhs_digit = rhs as u64;

        quotient[i] = (lhs_digit / rhs_digit) as u32;
        remainder = lhs_digit % rhs_digit;
    }

    trim(&mut quotient);
    (quotient, vec![remainder as u32])
}

fn mul_sub(u: &mut [u32], v: &[u32], q: u64) -> u64 {
    let mut mul_carry = 0;
    let mut sub_carry = 0;

    for i in 0..v.len() {
        let product = q * (v[i] as u64) + mul_carry;
        let sum = (BASE + (u[i] as u64)) - (product & 0xffffffff) - sub_carry;

        u[i] = (sum & 0xffffffff) as u32;

        mul_carry = product >> 32;
        sub_carry = 1 - (sum >> 32);
    }

    let sum = (BASE + (u[v.len()] as u64)) - mul_carry - sub_carry;
    u[v.len()] = (sum & 0xffffffff) as u32;

    1 - (sum >> 32)
}

fn normalize(lhs: &[u32], rhs: &[u32]) -> (u32, Vec<u32>, Vec<u32>) {
    let shift = rhs[rhs.len() - 1].leading_zeros();

    let mut u = lhs.to_vec();
    u.push(0);

    let mut v = rhs.to_vec();

    if shift != 0 {
        lshift(&mut u, shift);
        lshift(&mut v, shift);
    }

    (shift, u, v)
}

fn denormalize(mut u: Vec<u32>, shift: u32) -> Vec<u32> {
    if shift != 0 {
        rshift(&mut u, shift);
    }

    u
}

fn div_by_many(lhs: &[u32], rhs: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let mut quotient = vec![0; lhs.len() - rhs.len() + 1];

    let (shift, mut u, v) = normalize(lhs, rhs);

    let m = lhs.len();
    let n = rhs.len();

    for j in (0..m - n + 1).rev() {
        let lhs_digit = ((u[j + n] as u64) << 32) + (u[j + n - 1] as u64);
        let rhs_digit = v[n - 1] as u64;

        let mut q = lhs_digit / rhs_digit;
        let mut r = lhs_digit % rhs_digit;

        while q >= BASE || q * (v[n - 2] as u64) > (r << 32) + (u[j + n - 2] as u64) {
            q -= 1;
            r += v[n - 1] as u64;

            if r >= BASE {
                break;
            }
        }

        if mul_sub(&mut u[j..j + n + 1], &v, q) != 0 {
            q -= 1;
            add(&mut u[j..j + n + 1], &v);
        }

        quotient[j] = q as u32;
    }

    let mut remainder = denormalize(u, shift);

    trim(&mut remainder);
    trim(&mut quotient);

    (quotient, remainder)
}

fn div(lhs: &[u32], rhs: &[u32]) -> (Vec<u32>, Vec<u32>) {
    if rhs.len() == 0 {
        panic!("cannot divide by zero");
    }

    if lhs.len() < rhs.len() {
        return (Vec::new(), lhs.to_vec());
    }

    if rhs.len() == 1 {
        return div_by_one(lhs, rhs[0]);
    }

    return div_by_many(lhs, rhs);
}

trait FromU64 {
    fn from_u64(value: u64) -> Self;
}

impl FromU64 for u32 {
    fn from_u64(value: u64) -> Self {
        value as u32
    }
}

impl FromU64 for u8 {
    fn from_u64(value: u64) -> Self {
        value as u8
    }
}

fn radix_convert<T: FromU64>(from: &mut [u32], from_base: u64, to_base: u64) -> Vec<T> {
    let mut to = Vec::new();
    let mut len = from.len();

    while len > 0 {
        let mut remainder = 0;

        for i in (0..len).rev() {
            let lhs = (remainder * from_base) + (from[i] as u64);
            from[i] = (lhs / to_base) as u32;
            remainder = lhs % to_base;
        }

        while len > 0 && from[len - 1] == 0 {
            len -= 1;
        }

        to.push(T::from_u64(remainder));
    }

    to
}

fn base10_from_str(decimal: &str) -> Vec<u32> {
    decimal.as_bytes().iter().rev().map(|digit| (digit - b'0') as u32).collect()
}

pub struct BigNum {
    digits: Vec<u32>,
}

impl BigNum {
    pub fn new(num: u64) -> Self {
        let mut digits = vec![(num & 0xffffffff) as u32, (num >> 32) as u32];
        trim(&mut digits);
        BigNum { digits: digits }
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

        trim(&mut digits);
        BigNum { digits: digits }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for digit in self.digits.iter() {
            for i in 0..4 {
                bytes.push(((*digit >> (8 * i)) & 0xff) as u8);
            }
        }

        trim(&mut bytes);

        bytes.reverse();
        bytes
    }

    pub fn from_decimal(decimal: &str) -> Self {
        let mut base10 = base10_from_str(decimal);
        let digits = radix_convert(&mut base10, 10, BASE);

        BigNum { digits: digits }
    }

    pub fn to_decimal(&self) -> String {
        let mut digits = self.digits.clone();
        let mut base10 = radix_convert(&mut digits, BASE, 10);

        for digit in base10.iter_mut() {
            *digit += b'0';
        }

        if base10.len() == 0 {
            base10.push(b'0');
        }

        base10.reverse();
        String::from_utf8(base10).unwrap()
    }

    pub fn clone(&self) -> Self {
        BigNum { digits: self.digits.clone() }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let mut result = self.digits.clone();
        zeropad(&mut result, max(self.digits.len(), rhs.digits.len()) + 1);

        add(&mut result, &rhs.digits);

        trim(&mut result);
        BigNum { digits: result }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        let mut result = self.digits.clone();
        zeropad(&mut result, max(self.digits.len(), rhs.digits.len()));

        if sub(&mut result, &rhs.digits) != 0 {
            panic!("cannot subtract larger value");
        }

        trim(&mut result);
        BigNum { digits: result }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut result = mul(&self.digits, &rhs.digits);

        trim(&mut result);
        BigNum { digits: result }
    }

    pub fn div(&self, rhs: &Self) -> (Self, Self) {
        let (quotient, remainder) = div(&self.digits, &rhs.digits);
        (BigNum { digits: quotient }, BigNum { digits: remainder })
    }

    pub fn modexp(&self, exponent: &Self, modulus: &Self) -> Self {
        let mut result = vec![1];
        let mut saw_bit = false;

        for digit in exponent.digits.iter().rev() {
            for i in (0..32).rev() {
                let is_set = (digit & (1 << i)) != 0;

                if saw_bit {
                    result = div(&mul(&result, &result), &modulus.digits).1;

                    if is_set {
                        result = div(&mul(&result, &self.digits), &modulus.digits).1;
                    }
                } else if is_set {
                    saw_bit = true;
                    result = div(&mul(&result, &self.digits), &modulus.digits).1;
                }
            }
        }

        BigNum { digits: result }
    }

    fn modsub(tuple: (Self, Self), quotient: &Self, modulus: &Self) -> (Self, Self) {
        let (new, mut old) = tuple;

        let product = new.mul(&quotient);
        while old < product {
            old = old.add(&modulus);
        }

        (old.sub(&product), new)
    }

    pub fn modinv(&self, modulus: &Self) -> Self {
        let mut r = (modulus.clone(), self.clone());
        let mut s = (BigNum::new(0), BigNum::new(1));
        let mut t = (BigNum::new(1), BigNum::new(0));

        while r.0.digits.len() != 0 {
            let quotient = r.1.div(&r.0).0;

            r = BigNum::modsub(r, &quotient, &modulus);
            s = BigNum::modsub(s, &quotient, &modulus);
            t = BigNum::modsub(t, &quotient, &modulus);
        }

        s.1.div(modulus).1
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
        let a = BigNum::new(1);
        let b = a.add(&a);
        assert_eq!(&b.to_decimal(), "2");
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
        let a = BigNum::new(1);
        let b = a.sub(&a);
        assert_eq!(&b.to_decimal(), "0");
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
    #[should_panic]
    fn sub_negative2() {
        let a = BigNum::from_bytes(&decode("01").unwrap());
        let b = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
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

    #[test]
    fn from_decimal2() {
        let a = BigNum::from_decimal("000001");
        assert_eq!(a.to_decimal(), "1");
    }

    #[test]
    fn from_decimal3() {
        let a = BigNum::from_decimal("11");
        assert_eq!(a.to_decimal(), "11");
    }

    #[test]
    fn from_decimal4() {
        let a = BigNum::from_decimal("111");
        assert_eq!(a.to_decimal(), "111");
    }

    #[test]
    #[should_panic]
    fn div_zero1() {
        let a = BigNum::new(1);
        let b = BigNum::new(0);
        a.div(&b);
    }

    #[test]
    fn div_zero2() {
        let a = BigNum::new(0);
        let b = BigNum::new(123);

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "0");
        assert_eq!(r.to_decimal(), "0");
    }

    #[test]
    fn div_small1() {
        let a = BigNum::new(1234);
        let b = BigNum::new(1 << 32);

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "0");
        assert_eq!(r.to_decimal(), "1234");
    }

    #[test]
    fn div_small2() {
        let a = BigNum::new(999);
        let b = BigNum::new(11);

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "90");
        assert_eq!(r.to_decimal(), "9");
    }

    #[test]
    fn div_big1() {
        let a = BigNum::from_decimal("18446744073709551615");
        let b = BigNum::from_decimal("4294967296");

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "4294967295");
        assert_eq!(r.to_decimal(), "4294967295");
    }

    #[test]
    fn div_big2() {
        let a = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());
        let b = BigNum::from_bytes(&decode("ffffffffffffffff").unwrap());

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "1");
        assert_eq!(r.to_decimal(), "0");
    }

    #[test]
    fn div_big3() {
        let a = BigNum::from_decimal("79228162514264337593543950335");
        let b = BigNum::from_decimal("10995116277759");

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "7205759403793448");
        assert_eq!(r.to_decimal(), "10555311627303");
    }

    #[test]
    fn div_big4() {
        let a = BigNum::from_bytes(&decode("800000000000000000000003").unwrap());
        let b = BigNum::from_bytes(&decode("200000000000000000000001").unwrap());

        let (q, r) = a.div(&b);

        assert_eq!(q.to_decimal(), "3");
        assert_eq!(r.to_decimal(), "9903520314283042199192993792");
    }

    #[test]
    fn modexp1() {
        let n = BigNum::new(2);
        let e = BigNum::new(0);
        let m = BigNum::new(10);
        let r = n.modexp(&e, &m);

        assert_eq!(r.to_decimal(), "1");
    }

    #[test]
    fn modexp2() {
        let n = BigNum::new(2);
        let e = BigNum::new(1024);
        let m = BigNum::new(10);
        let r = n.modexp(&e, &m);

        assert_eq!(r.to_decimal(), "6");
    }
}
