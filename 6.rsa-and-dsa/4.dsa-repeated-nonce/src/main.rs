#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;

struct Signature {
    s: BigNum,
    r: BigNum,
    h: BigNum,
}

impl Signature {
    fn new(s: &str, r: &str, h: &str) -> Signature {
        Signature {
            r: BigNum::from_decimal(r),
            s: BigNum::from_decimal(s),
            h: BigNum::from_bytes(&pals::hex::decode(h).unwrap()),
        }
    }
}

fn get_k(a: &Signature, b: &Signature, q: &BigNum) -> BigNum {
    let mut h1 = a.h.clone();
    let h2 = &b.h;

    while &h1 < h2 {
        h1 = h1.add(q);
    }

    let mut s1 = a.s.clone();
    let s2 = &b.s;

    while &s1 < s2 {
        s1 = s1.add(&q);
    }

    h1.sub(h2).mul(&s1.sub(s2).modinv(q)).div(q).1
}

fn get_x(q: &BigNum, r: &BigNum, s: &BigNum, h: &BigNum, k: &BigNum) -> BigNum {
    let mut sk = s.mul(k);

    while &sk < h {
        sk = sk.add(q);
    }

    sk.sub(h).mul(&r.modinv(q)).div(q).1
}

fn get_signatures() -> Vec<Signature> {
    let mut signatures = Vec::new();

    signatures.push(Signature::new("1267396447369736888040262262183731677867615804316",
                                   "1105520928110492191417703162650245113664610474875",
                                   "a4db3de27e2db3e5ef085ced2bced91b82e0df19"));

    signatures.push(Signature::new("29097472083055673620219739525237952924429516683",
                                   "51241962016175933742870323080382366896234169532",
                                   "a4db3de27e2db3e5ef085ced2bced91b82e0df19"));

    signatures.push(Signature::new("277954141006005142760672187124679727147013405915",
                                   "228998983350752111397582948403934722619745721541",
                                   "21194f72fe39a80c9c20689b8cf6ce9b0e7e52d4"));

    signatures.push(Signature::new("1013310051748123261520038320957902085950122277350",
                                   "1099349585689717635654222811555852075108857446485",
                                   "1d7aaaa05d2dee2f7dabdc6fa70b6ddab9c051c5"));

    signatures.push(Signature::new("203941148183364719753516612269608665183595279549",
                                   "425320991325990345751346113277224109611205133736",
                                   "06bc188db6e9e6c7d796f7fdd7fa411776d7a9ff"));

    signatures.push(Signature::new("502033987625712840101435170279955665681605114553",
                                   "486260321619055468276539425880393574698069264007",
                                   "5ff4d4e8be2f8aae8a5bfaabf7408bd7628f43c9"));

    signatures.push(Signature::new("1133410958677785175751131958546453870649059955513",
                                   "537050122560927032962561247064393639163940220795",
                                   "07d9abd18bbecdaa93650ecc4da1b9fcae911412"));

    signatures.push(Signature::new("559339368782867010304266546527989050544914568162",
                                   "826843595826780327326695197394862356805575316699",
                                   "88b9e184393408b133efef59fcef85576d69e249"));

    signatures.push(Signature::new("1021643638653719618255840562522049391608552714967",
                                   "1105520928110492191417703162650245113664610474875",
                                   "d22804c4899b522b23eda34d2137cd8cc22b9ce8"));

    signatures.push(Signature::new("506591325247687166499867321330657300306462367256",
                                   "51241962016175933742870323080382366896234169532",
                                   "bc7ec371d951977cba10381da08fe934dea80314"));

    signatures.push(Signature::new("458429062067186207052865988429747640462282138703",
                                   "228998983350752111397582948403934722619745721541",
                                   "d6340bfcda59b6b75b59ca634813d572de800e8f"));

    signatures
}

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("800000000000000089e1855218a0e7dac38136ffafa72\
                                                   eda7859f2171e25e65eac698c1702578b07dc2a1076da\
                                                   241c76c62d374d8389ea5aeffd3226a0530cc565f3bf6\
                                                   b50929139ebeac04f48c3c84afb796d61e5a4f9a8fda8\
                                                   12ab59494232c7d2b4deb50aa18ee9e132bfa85ac4374\
                                                   d7f9091abc3d015efc871a584471bb1")
                                    .unwrap());

    let q = BigNum::from_bytes(&pals::hex::decode("f4f47f05794b256174bba6e9b396a7707e563c5b")
                                    .unwrap());

    let g = BigNum::from_bytes(&pals::hex::decode("5958c9d3898b224b12672c0b98e06c60df923cb8bc999\
                                                   d119458fef538b8fa4046c8db53039db620c094c9fa07\
                                                   7ef389b5322a559946a71903f990f1f7e0e025e2d7f7c\
                                                   f494aff1a0470f5b64c36b625a097f1651fe775323556\
                                                   fe00b3608c887892878480e99041be601a62166ca6894\
                                                   bdd41a7054ec89f756ba9fc95302291")
                                    .unwrap());

    let y = BigNum::from_bytes(&pals::hex::decode("2d026f4bf30195ede3a088da85e398ef869611d0f68f0\
                                                   713d51c9c1a3a26c95105d915e2d8cdf26d056b86b8a7\
                                                   b85519b1c23cc3ecdc6062650462e3063bd179c2a6581\
                                                   519f674a61f1d89a1fff27171ebc1b93d4dc57bceb7ae\
                                                   2430f98a6a4d83d8279ee65d71c1203d2c96d65ebbf7c\
                                                   ce9d32971c3de5084cce04a2e147821")
                                    .unwrap());

    let signatures = get_signatures();

    for s1 in signatures.iter() {
        for s2 in signatures.iter() {
            if s1.h == s2.h {
                continue;
            }

            let k = get_k(&s1, &s2, &q);
            let x = get_x(&q, &s1.r, &s1.s, &s1.h, &k);

            if y == g.modexp(&x, &p) {
                let hex = pals::hex::encode(&x.to_bytes());
                let sha1 = pals::hex::encode(&pals::sha1::hash(hex.as_bytes()));
                println!("private key is: {} (sha1: {})", hex, sha1);
                return;
            }
        }
    }
}
