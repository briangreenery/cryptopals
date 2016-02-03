#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;
use rand::Rng;
use std::collections::BTreeSet;

struct RSA {
    n: BigNum,
    e: BigNum,
    d: BigNum,
    size: usize,
}

impl RSA {
    fn new(p: &BigNum, q: &BigNum) -> RSA {
        let one = BigNum::new(1);
        let n = p.mul(&q);
        let et = p.sub(&one).mul(&q.sub(&one));
        let e = BigNum::new(3);
        let d = e.modinv(&et);
        let size = n.to_bytes().len();
        RSA {
            n: n,
            e: e,
            d: d,
            size: size,
        }
    }

    fn encrypt(&self, plain: &BigNum) -> BigNum {
        plain.modexp(&self.e, &self.n)
    }

    fn decrypt(&self, cipher: &BigNum) -> BigNum {
        cipher.modexp(&self.d, &self.n)
    }
}

fn pad(num: &BigNum, size: usize) -> Vec<u8> {
    let bytes = num.to_bytes();

    if bytes.len() >= size {
        return bytes;
    }

    let mut padded = vec![0; size - bytes.len()];
    padded.extend(&bytes);
    padded
}

fn pkcs15_pad(data: &[u8], size: usize) -> Vec<u8> {
    let mut padding = vec![0; size];

    if data.len() + 11 > size {
        panic!("data too large");
    }

    padding[1] = 2;

    let mut rng = rand::thread_rng();

    for i in 2..size - data.len() - 1 {
        while padding[i] == 0 {
            padding[i] = rng.gen();
        }
    }

    for (i, byte) in data.iter().enumerate() {
        padding[size - data.len() + i] = *byte;
    }

    padding
}

fn pkcs15_unpad(padding: &[u8]) -> Option<&[u8]> {
    if padding.len() < 11 {
        return None;
    }

    if padding[0] != 0 {
        return None;
    }

    if padding[1] != 2 {
        return None;
    }

    for i in 2..10 {
        if padding[i] == 0 {
            return None;
        }
    }

    for i in 10..padding.len() {
        if padding[i] == 0 {
            return Some(&padding[i + 1..]);
        }
    }

    return None;
}

fn pkcs15_is_valid(key: &RSA, ciphertext: &BigNum) -> bool {
    let plain = pad(&key.decrypt(&ciphertext), key.size);
    pkcs15_unpad(&plain).is_some()
}

fn floor(div_result: (BigNum, BigNum)) -> BigNum {
    div_result.0
}

fn ceil(div_result: (BigNum, BigNum)) -> BigNum {
    if div_result.1 == BigNum::new(0) {
        div_result.0
    } else {
        div_result.0.add(&BigNum::new(1))
    }
}

fn find_next(key: &RSA, c0: &BigNum, start: &BigNum) -> BigNum {
    let one = BigNum::new(1);
    let mut s = start.clone();

    loop {
        let encrypted = s.modexp(&key.e, &key.n).mul(c0).div(&key.n).1;

        if pkcs15_is_valid(key, &encrypted) {
            return s;
        }

        s = s.add(&one);
    }
}

fn find_in_range(key: &RSA,
                 c0: &BigNum,
                 s: &BigNum,
                 b2: &BigNum,
                 b3: &BigNum,
                 n: &BigNum,
                 range: &(BigNum, BigNum))
                 -> BigNum {
    let one = BigNum::new(1);
    let &(ref a, ref b) = range;

    let mut r = ceil(BigNum::new(2).mul(&b.mul(s).sub(b2)).div(n));

    loop {
        let rn = r.mul(n);

        let s_min = ceil(b2.add(&rn).div(b));
        let s_max = floor(b3.add(&rn).div(a));

        let mut s = s_min.clone();

        while s <= s_max {
            let encrypted = s.modexp(&key.e, &key.n).mul(&c0).div(&key.n).1;

            if pkcs15_is_valid(&key, &encrypted) {
                return s;
            }

            s = s.add(&one);
        }

        r = r.add(&one);
    }
}

fn compute_ranges(ranges: &BTreeSet<(BigNum, BigNum)>,
                  s: &BigNum,
                  b2: &BigNum,
                  b3: &BigNum,
                  n: &BigNum)
                  -> BTreeSet<(BigNum, BigNum)> {
    let one = BigNum::new(1);
    let mut new_ranges = BTreeSet::<(BigNum, BigNum)>::new();

    for &(ref a, ref b) in ranges {
        let r_min = ceil(a.mul(s).sub(b3).add(&one).div(n));
        let r_max = floor(b.mul(s).sub(b2).div(n));

        let mut r = r_min.clone();

        while r <= r_max {
            let rn = r.mul(n);

            let x = ceil(b2.add(&rn).div(s));
            let y = floor(b3.sub(&one).add(&rn).div(s));

            let range = (std::cmp::max(a.clone(), x), std::cmp::min(b.clone(), y));

            if range.0 <= range.1 {
                new_ranges.insert(range);
            }

            r = r.add(&one);
        }
    }

    for &(ref a, ref b) in &new_ranges {
        println!("range = ({}, {})", a.to_decimal(), b.to_decimal());
    }

    new_ranges
}

fn first_range(ranges: &BTreeSet<(BigNum, BigNum)>) -> Option<&(BigNum, BigNum)> {
    for range in ranges {
        return Some(range);
    }

    None
}

fn found_solution(ranges: &BTreeSet<(BigNum, BigNum)>) -> bool {
    if ranges.len() != 1 {
        return false;
    }

    for &(ref a, ref b) in ranges {
        if a == b {
            return true;
        }
    }

    return false;
}

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("FFA4D5B0EE560E34FD251116B5EFC497D3EB2094629EF\
                                                   4D9887750A0A1684C71A04E53608440CF5012F3C31A44\
                                                   06AE4D")
                                    .unwrap());
    let q = BigNum::from_bytes(&pals::hex::decode("C37E2F10501B1B9CD5CE9EFFF22A5E484D7596D8209DE\
                                                   5C62425643B1E0897AAD52DCF651D44ED5C1F69057970\
                                                   2E7A37")
                                    .unwrap());
    let key = RSA::new(&p, &q);

    let b = BigNum::new(2).modexp(&BigNum::new(8 * (key.size - 2) as u64), &key.n);
    let b2 = b.mul(&BigNum::new(2));
    let b3 = b.mul(&BigNum::new(3));
    let n = &key.n;

    let c0 = key.encrypt(&BigNum::from_bytes(&pkcs15_pad("cats are cute".as_bytes(), key.size)));

    let mut ranges = BTreeSet::<(BigNum, BigNum)>::new();
    ranges.insert((b2.clone(), b3.sub(&BigNum::new(1))));

    let mut s = find_next(&key, &c0, &ceil(n.div(&b3)));

    loop {
        println!("found s = {}", s.to_decimal());
        ranges = compute_ranges(&ranges, &s, &b2, &b3, &n);

        if found_solution(&ranges) {
            break;
        }

        if ranges.len() == 0 {
            panic!("no solution found!");
        } else if ranges.len() == 1 {
            s = find_in_range(&key, &c0, &s, &b2, &b3, &n, first_range(&ranges).unwrap());
        } else {
            s = find_next(&key, &c0, &s.add(&BigNum::new(1)));
        }
    }

    let plain = pad(&first_range(&ranges).unwrap().0, key.size);
    let decrypted = pkcs15_unpad(&plain).unwrap();

    println!("decrypted: {}",
             String::from_utf8(decrypted.to_vec()).unwrap());
}
