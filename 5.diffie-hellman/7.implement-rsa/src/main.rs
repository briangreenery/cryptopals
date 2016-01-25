#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use pals::BigNum;

fn main() {
    let p = BigNum::from_bytes(&pals::hex::decode("D12FCEC212D49FD67EFA55FD45B3CD76D34B711490B6F\
                                                   2CD5C893E8C2D6CCC799C46E6B240A050F70C137A0561\
                                                   D6CA73D689F57C75B6948009EF5EA4DFFC5F7AB8FE1BC\
                                                   B320EC138A1DA8FFF73D0279CDC50DE23AE3D1278F14E\
                                                   60C7B826BB6C6D3051646A33B93439143DA900F7C86B7\
                                                   3AA038B43416F15E1B1D9433900AD4FFA13223B37CFA0\
                                                   D1F4375737668C096F91A420B6A67824ABE13AEC3D4D6\
                                                   8ACDF75611F335299F37FCCCA7CCEA8CA2EE9E345AD16\
                                                   1B5E8BCDC28311799B68073409391DF38B4F198B542D1\
                                                   31575D9060C2FCF41BA0615AF602323FDF21D72D86727\
                                                   40EF9F5A51E5545BDCF33BAC239BAD0E06AC933305ABE\
                                                   DDA58951C234C6B9F")
                                    .unwrap());

    let q = BigNum::from_bytes(&pals::hex::decode("DF9037B7BD278123B7C20F38C7B7205C0F9A642840930\
                                                   295E84B4799A633917FC9AC65CC89AE6F52158B7C96A9\
                                                   D7BC0F2F78D5EA823115C6B4B63CC2CA35344BFE47D55\
                                                   D56F266DFB70381F348838B8F6631C3D97B2A64FB70BC\
                                                   170A29FC0C0F8593EED9FB969442ECC63C9EB54841BE8\
                                                   5EB96A954EBFF2B9B867B84DDE74D72E0FEAC9FCE3867\
                                                   56F0BCCBD5AE4D0F70BD1A6EDB82C9B94D9659EAE799F\
                                                   29F0C06DCCBAA870C4A5ABB39CC7DAB60F33CD64C090D\
                                                   DE4D6DB7498F1C3BA23143E8EE4B0861E37007BC0C5FF\
                                                   4FA554B7F89010881A3A9B5404C95FA059BB240964030\
                                                   54592CF7AE46510FD49627A2D74C3C2D954945FCC5666\
                                                   EAAE6077A9AABBD0B")
                                    .unwrap());

    let n = p.mul(&q);

    let et = p.sub(&BigNum::new(1)).mul(&q.sub(&BigNum::new(1)));
    let e = BigNum::new(3);
    let d = e.modinv(&et);

    let text = "hodor is the bomb diggity";
    println!("plaintext: {}", text);

    let encrypted = BigNum::from_bytes(text.as_bytes()).modexp(&e, &n);
    println!("encrypted: {}", pals::hex::encode(&encrypted.to_bytes()));

    let decrypted = encrypted.modexp(&d, &n);
    println!("decrypted: {}",
             String::from_utf8(decrypted.to_bytes()).unwrap());
}
