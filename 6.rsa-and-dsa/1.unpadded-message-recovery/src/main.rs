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

    fn encrypt(&self, plain: &[u8]) -> BigNum {
        let size = self.n.to_bytes().len();

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

    let plaintext = "cats are cute";
    let ciphertext = key.encrypt(plaintext.as_bytes());

    let s = BigNum::new(42);
    let ciphertext2 = s.modexp(&key.e, &key.n).mul(&ciphertext).div(&key.n).1;

    println!("ciphertext = {}\n",
             pals::hex::encode(&ciphertext.to_bytes()));
    println!("modified   = {}\n",
             pals::hex::encode(&ciphertext2.to_bytes()));

    let decrypted2 = BigNum::from_bytes(&key.decrypt(&ciphertext2));
    let decrypted = decrypted2.mul(&s.modinv(&key.n)).div(&key.n).1;

    println!("decrypted: {}",
             String::from_utf8(decrypted.to_bytes()).unwrap());
}
