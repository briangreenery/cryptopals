pub struct BigNum {
    digits: Vec<u64>,
}

impl BigNum {
    pub fn new() -> BigNum {
        BigNum { digits: Vec::new() }
    }

    pub fn from_hex(hex: &str) -> Result<BigNum, super::hex::Error> {
        let mut decoded = try!(super::hex::decode(&hex));
        decoded.reverse();

        let mut digits = Vec::new();

        for chunk in decoded.chunks(8) {
            let mut digit = 0;

            for i in (0..8).rev() {
                if i < chunk.len() {
                    digit *= 256;
                    digit += chunk[i] as u64;
                }
            }

            digits.push(digit);
        }

        Ok(BigNum { digits: digits })
    }

    pub fn to_hex(&self) -> String {
        let mut bytes = Vec::new();

        for digit in self.digits.iter() {
            let mut value = *digit;

            for _ in 0..8 {
                bytes.push((value % 256) as u8);
                value /= 256;
            }
        }

        while bytes.len() > 0 && bytes[bytes.len() - 1] == 0 {
            bytes.pop();
        }

        bytes.reverse();
        super::hex::encode(&bytes)
    }

    fn add_to(&mut self, index: usize, amount: u64) -> u64 {
        let value = self.digits[index];

        if value > u64::max_value() - amount {
            self.digits[index] = value - (u64::max_value() - amount) - 1;
            1
        } else {
            self.digits[index] = value + amount;
            0
        }
    }

    pub fn add(&mut self, amount: &BigNum) {
        let mut carry = 0;
        let mut index = 0;

        while carry > 0 || index < amount.digits.len() {
            if self.digits.len() <= index {
                self.digits.push(0);
            }

            carry = self.add_to(index, carry);

            if index < amount.digits.len() {
                carry += self.add_to(index, amount.digits[index]);
            }
            
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_small() {
        let a = BigNum::from_hex("0000000000000000000001").unwrap();
        assert_eq!(a.to_hex(), "01");
    }

    #[test]
    fn hex_large() {
        let a = BigNum::from_hex("0100000000000000000000").unwrap();
        assert_eq!(a.to_hex(), "0100000000000000000000");
    }

    #[test]
    fn add() {
        let mut a = BigNum::from_hex("01").unwrap();
        let b = BigNum::from_hex("01").unwrap();

        a.add(&b);
        assert_eq!(a.to_hex(), "02");
    }

    #[test]
    fn add_with_carry1() {
        let mut a = BigNum::from_hex("ffffffffffffffff").unwrap();
        let b = BigNum::from_hex("01").unwrap();

        a.add(&b);
        assert_eq!(a.to_hex(), "010000000000000000");
    }

    #[test]
    fn add_with_carry2() {
        let mut a = BigNum::from_hex("ffffffffffffffff").unwrap();
        let b = BigNum::from_hex("ffffffffffffffff").unwrap();

        a.add(&b);
        assert_eq!(a.to_hex(), "01fffffffffffffffe");
    }
}
