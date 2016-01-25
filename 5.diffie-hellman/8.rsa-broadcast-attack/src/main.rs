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

    fn decrypt(&self, cihper: &BigNum) -> Vec<u8> {
        cihper.modexp(&self.d, &self.n).to_bytes()
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

    while low < high {
        let mid = high.add(&low).div(&two).0;
        let mid_cubed = cube(&mid);

        if &mid_cubed > n {
            high = mid;
        } else if &mid_cubed < n {
            low = mid.add(&one);
        } else {
            return mid;
        }
    }

    panic!("cube root not found");
}

fn main() {
    let p1 = BigNum::from_bytes(&pals::hex::decode("EA23B38794C003AA9F98EE8C64F0945088CDE3947B89\
                                                    234E6C44F59535F21E6D554B3F4DA448FAC3D397F0A8\
                                                    5F7A6C745D906458FD92CFB92945CDA18A05F7F238A3\
                                                    E3D1D45BDD6E1D01015E2005A77BBF8F8D71953696FF\
                                                    6A9FEB7966CEC65955E21A7884ED0F9FD5F49D9929B9\
                                                    F6C7A1D0D7DAD6423637DDEE2A569A2C504B")
                                     .unwrap());

    let q1 = BigNum::from_bytes(&pals::hex::decode("D545070971EC17CF8473E1D10A92184C9925D147B131\
                                                    7ED7987F2B17AB00EA476454ECD0B4E9EA5E18EB6996\
                                                    35FF637CE07350117A5D7307BC0C328D53E685231EC2\
                                                    23A481F4739BE14F041219D3B9A05663A04FD6ECE203\
                                                    D92474321197FB9D60265B5130E7478AEBFBFCD48BDA\
                                                    550C086147BFB14E0A9F4CE53019914F9BAB")
                                     .unwrap());

    let p2 = BigNum::from_bytes(&pals::hex::decode("F660ED66498A391DD81EDF43C83FAE2E70B2ADEEF6F2\
                                                    5615C882139D13356E0D4CE8B46204C2215DB4829FBE\
                                                    F269A5680F898FB356909E79A809A763F74D0CC430D7\
                                                    7373FF862DEE4E399056CE4D72F01F97B995C8AA90B8\
                                                    9FF4A54D2CFF0E06A696A6470FAB32FB109636CA1F4D\
                                                    30A28A22A286FC82D0AA4963268D0BBC8C8B")
                                     .unwrap());

    let q2 = BigNum::from_bytes(&pals::hex::decode("EA906756903694D25E97D961D8033206DD833FE67DDF\
                                                    81DB15A2A50261E7DAFBBFD54A91AA665E7E3F034700\
                                                    065D6E6C049C2702AB219A0B41F6FAF685A31688CB6E\
                                                    89D8AEF9ECB1C42062B165D0E3634C8C6E9BEF2403BD\
                                                    2DECFCE161DF3F443707A4589F1589CA05D49B3DE89A\
                                                    38199C7BF4F672E630CFC3DA3F73AFC429BF")
                                     .unwrap());

    let p3 = BigNum::from_bytes(&pals::hex::decode("C05B51059474ED23D6957962734F7639AE8E7ACBCD4F\
                                                    CB42F820E2EC9167BD261B705ED975315EA489A25748\
                                                    67E98926F4F7E9CF1DD9FA6C0D172737A2D83E81B022\
                                                    E1056D3847A0A169E64FA31F817775DAFA77455E2154\
                                                    F0247B6ABC720E77BF2CF6B4FB6DCA6D4B05D35350F1\
                                                    942ECEEED5DC82740C24EAB59A2D86D025E3")
                                     .unwrap());

    let q3 = BigNum::from_bytes(&pals::hex::decode("C704F04F0542DD0D23009B76A178E2E12B1482F0B377\
                                                    5B87F1398AA24E282465DE8B0BDE2E17320165477361\
                                                    E1106D1EA49AF26069223ED704E53863EB19D5369A7F\
                                                    6D0FDB4AEBABA1CF6AC9C6AE4BF8063B213450D7E481\
                                                    C03EF036B3039C570763E9D3885FC6BF3D3390745A71\
                                                    A8E3DF98ECF7B6FED1985402F96719ECB7A1")
                                     .unwrap());

    let key1 = RSA::new(&p1, &q1);
    let key2 = RSA::new(&p2, &q2);
    let key3 = RSA::new(&p3, &q3);

    let text = "hodor is the bomb diggity!!@";
    println!("plaintext: {}", text);

    let encrypted1 = key1.encrypt(text.as_bytes());
    let encrypted2 = key2.encrypt(text.as_bytes());
    let encrypted3 = key3.encrypt(text.as_bytes());

    println!("\nencrypted as:\n");
    println!("{}\n", pals::hex::encode(&encrypted1.to_bytes()));
    println!("{}\n", pals::hex::encode(&encrypted2.to_bytes()));
    println!("{}\n", pals::hex::encode(&encrypted3.to_bytes()));

    let n1 = &key1.n;
    let n2 = &key2.n;
    let n3 = &key3.n;

    let n12 = n1.mul(n2);
    let n23 = n2.mul(&n3);
    let n13 = n1.mul(&n3);

    let n = n1.mul(n2).mul(n3);

    let x1 = encrypted1.mul(&n23).mul(&n23.modinv(n1));
    let x2 = encrypted2.mul(&n13).mul(&n13.modinv(n2));
    let x3 = encrypted3.mul(&n12).mul(&n12.modinv(n3));

    let x = x1.add(&x2).add(&x3).div(&n).1;

    let recovered = cube_root(&x).to_bytes();
    println!("recovered: {}", String::from_utf8(recovered).unwrap());
}
