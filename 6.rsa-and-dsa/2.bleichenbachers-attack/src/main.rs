#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;

struct RSA {
    n: BigNum,
    e: BigNum,
    d: BigNum,
}

impl RSA {
    fn new(p: &BigNum, q: &BigNum) -> RSA {
        let n = p.mul(&q);
        let et = p.sub(&BigNum::new(1)).mul(&q.sub(&BigNum::new(1)));
        let e = BigNum::new(3);
        let d = e.modinv(&et);
        RSA { n: n, e: e, d: d }
    }

    fn len(&self) -> usize {
        self.n.to_bytes().len()
    }

    fn encrypt(&self, plain: &[u8]) -> BigNum {
        let size = self.len();

        if plain.len() > size {
            panic!("text to encrypt is too big");
        }

        let mut bytes = plain.to_vec();
        while bytes.len() + 1 < size {
            bytes.push(0);
        }

        BigNum::from_bytes(&bytes).modexp(&self.e, &self.n)
    }

    fn decrypt(&self, cipher: &BigNum) -> Vec<u8> {
        cipher.modexp(&self.d, &self.n).to_bytes()
    }
}

fn cube(n: &BigNum) -> BigNum {
    n.mul(n).mul(n)
}

fn cube_root(n: &BigNum) -> BigNum {
    let one = BigNum::new(1);
    let two = BigNum::new(2);

    let mut low = BigNum::new(0);
    let mut high = n.add(&one);

    let mut last = BigNum::new(0);

    while low < high {
        let mid = high.add(&low).div(&two).0;
        let mid_cubed = cube(&mid);

        last = mid.clone();

        if &mid_cubed > n {
            high = mid;
        } else if &mid_cubed < n {
            low = mid.add(&one);
        } else {
            break;
        }
    }

    last
}

fn insecure_get_hash(bytes: &[u8]) -> Vec<u8> {
    let mut i = 0;

    assert_eq!(bytes[i], 0x01);
    i += 1;

    assert_eq!(bytes[i], 0xff);
    i += 1;

    while bytes[i] == 0xff {
        i += 1;
    }

    assert_eq!(bytes[i], 0x00);
    i += 1;

    for letter in b"ASN.1" {
        assert_eq!(bytes[i], *letter);
        i += 1;
    }

    bytes[i..i + 20].to_vec()
}

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("EA23B38794C003AA9F98EE8C64F0945088CDE3947B892\
                                                   34E6C44F59535F21E6D554B3F4DA448FAC3D397F0A85F\
                                                   7A6C745D906458FD92CFB92945CDA18A05F7F238A3E3D\
                                                   1D45BDD6E1D01015E2005A77BBF8F8D71953696FF6A9F\
                                                   EB7966CEC65955E21A7884ED0F9FD5F49D9929B9F6C7A\
                                                   1D0D7DAD6423637DDEE2A569A2C504B")
                                    .unwrap());

    let q = BigNum::from_bytes(&pals::hex::decode("D545070971EC17CF8473E1D10A92184C9925D147B1317\
                                                   ED7987F2B17AB00EA476454ECD0B4E9EA5E18EB699635\
                                                   FF637CE07350117A5D7307BC0C328D53E685231EC223A\
                                                   481F4739BE14F041219D3B9A05663A04FD6ECE203D924\
                                                   74321197FB9D60265B5130E7478AEBFBFCD48BDA550C0\
                                                   86147BFB14E0A9F4CE53019914F9BAB")
                                    .unwrap());

    let key = RSA::new(&p, &q);

    let message = b"hi mom";
    let forged_hash = pals::sha1::hash(message).to_vec();

    // forge signature

    let mut forged_bytes = vec![0x01, 0xff, 0xff, 0xff, 0xff, 0x00, b'A', b'S', b'N', b'.', b'1'];
    forged_bytes.extend(&forged_hash);

    while forged_bytes.len() < 150 {
        forged_bytes.push(0);
    }

    let forged_num = BigNum::from_bytes(&forged_bytes);

    let mut root = cube_root(&forged_num);
    while cube(&root) < forged_num {
        root = root.add(&BigNum::new(1));
    }

    let mut forged_signature = Vec::new();
    for _ in 0..key.len() - root.to_bytes().len() {
        forged_signature.push(0);
    }

    forged_signature.extend(&root.to_bytes());

    // verify signature

    let signature = key.encrypt(&forged_signature).to_bytes();
    let hash = insecure_get_hash(&signature);

    println!("forged hash   {}", pals::hex::encode(&forged_hash));
    println!("verified hash {}", pals::hex::encode(&hash));
}
