#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

use rand::Rng;
use std::io;
use std::io::Write;

fn test_data() -> Vec<Vec<u8>> {
    let strings = ["SSBoYXZlIG1ldCB0aGVtIGF0IGNsb3NlIG9mIGRheQ==",
                   "Q29taW5nIHdpdGggdml2aWQgZmFjZXM=",
                   "RnJvbSBjb3VudGVyIG9yIGRlc2sgYW1vbmcgZ3JleQ==",
                   "RWlnaHRlZW50aC1jZW50dXJ5IGhvdXNlcy4=",
                   "SSBoYXZlIHBhc3NlZCB3aXRoIGEgbm9kIG9mIHRoZSBoZWFk",
                   "T3IgcG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==",
                   "T3IgaGF2ZSBsaW5nZXJlZCBhd2hpbGUgYW5kIHNhaWQ=",
                   "UG9saXRlIG1lYW5pbmdsZXNzIHdvcmRzLA==",
                   "QW5kIHRob3VnaHQgYmVmb3JlIEkgaGFkIGRvbmU=",
                   "T2YgYSBtb2NraW5nIHRhbGUgb3IgYSBnaWJl",
                   "VG8gcGxlYXNlIGEgY29tcGFuaW9u",
                   "QXJvdW5kIHRoZSBmaXJlIGF0IHRoZSBjbHViLA==",
                   "QmVpbmcgY2VydGFpbiB0aGF0IHRoZXkgYW5kIEk=",
                   "QnV0IGxpdmVkIHdoZXJlIG1vdGxleSBpcyB3b3JuOg==",
                   "QWxsIGNoYW5nZWQsIGNoYW5nZWQgdXR0ZXJseTo=",
                   "QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4=",
                   "VGhhdCB3b21hbidzIGRheXMgd2VyZSBzcGVudA==",
                   "SW4gaWdub3JhbnQgZ29vZCB3aWxsLA==",
                   "SGVyIG5pZ2h0cyBpbiBhcmd1bWVudA==",
                   "VW50aWwgaGVyIHZvaWNlIGdyZXcgc2hyaWxsLg==",
                   "V2hhdCB2b2ljZSBtb3JlIHN3ZWV0IHRoYW4gaGVycw==",
                   "V2hlbiB5b3VuZyBhbmQgYmVhdXRpZnVsLA==",
                   "U2hlIHJvZGUgdG8gaGFycmllcnM/",
                   "VGhpcyBtYW4gaGFkIGtlcHQgYSBzY2hvb2w=",
                   "QW5kIHJvZGUgb3VyIHdpbmdlZCBob3JzZS4=",
                   "VGhpcyBvdGhlciBoaXMgaGVscGVyIGFuZCBmcmllbmQ=",
                   "V2FzIGNvbWluZyBpbnRvIGhpcyBmb3JjZTs=",
                   "SGUgbWlnaHQgaGF2ZSB3b24gZmFtZSBpbiB0aGUgZW5kLA==",
                   "U28gc2Vuc2l0aXZlIGhpcyBuYXR1cmUgc2VlbWVkLA==",
                   "U28gZGFyaW5nIGFuZCBzd2VldCBoaXMgdGhvdWdodC4=",
                   "VGhpcyBvdGhlciBtYW4gSSBoYWQgZHJlYW1lZA==",
                   "QSBkcnVua2VuLCB2YWluLWdsb3Jpb3VzIGxvdXQu",
                   "SGUgaGFkIGRvbmUgbW9zdCBiaXR0ZXIgd3Jvbmc=",
                   "VG8gc29tZSB3aG8gYXJlIG5lYXIgbXkgaGVhcnQs",
                   "WWV0IEkgbnVtYmVyIGhpbSBpbiB0aGUgc29uZzs=",
                   "SGUsIHRvbywgaGFzIHJlc2lnbmVkIGhpcyBwYXJ0",
                   "SW4gdGhlIGNhc3VhbCBjb21lZHk7",
                   "SGUsIHRvbywgaGFzIGJlZW4gY2hhbmdlZCBpbiBoaXMgdHVybiw=",
                   "VHJhbnNmb3JtZWQgdXR0ZXJseTo=",
                   "QSB0ZXJyaWJsZSBiZWF1dHkgaXMgYm9ybi4="];

    strings.iter().map(|string| pals::base64::decode(string).unwrap()).collect()
}

fn get_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn printable(data: &[u8], key: &[u8]) -> String {
    let mut result = Vec::new();

    for (index, encrypted) in data.iter().enumerate() {
        let byte = encrypted ^ key[index];

        if byte >= 32 && byte <= 127 {
            result.push(byte);
        } else {
            result.push(b'.');
        }
    }

    String::from_utf8(result).unwrap()
}

fn any_bad(ciphers: &Vec<Vec<u8>>, key: &[u8], index: usize) -> bool {
    for cipher in ciphers {
        if cipher.len() <= index {
            continue;
        }

        let byte = cipher[index] ^ key[index];

        if byte > 127 {
            return true;
        }
    }

    false
}

fn main() {
    let key = pals::aes::random_key();
    let nonce = rand::thread_rng().gen();

    let ciphers: Vec<Vec<u8>> = test_data()
                                    .iter()
                                    .map(|plain| pals::aes::CTR::new(&key, nonce).apply(plain))
                                    .collect();

    let mut key = [0; 64];
    let mut index = 0;

    loop {
        println!("{}", pals::hex::encode(&key));

        for cipher in &ciphers {
            println!("{}", printable(&cipher, &key));
        }

        println!("----------");
        println!("n = next, p = prev, r = right, l = left");
        print!("Command for column {}: ", index);
        io::stdout().flush().unwrap();

        let guess = get_input();

        match guess.trim() {
            "n" => {
                let prev = key[index];

                loop {
                    if key[index] == 255 {
                        key[index] = 0;
                    } else {
                        key[index] += 1;
                    }

                    if !any_bad(&ciphers, &key, index) {
                        break;
                    }

                    if key[index] == prev {
                        break;
                    }
                }
            }
            "p" => {
                if key[index] == 0 {
                    key[index] = 255;
                } else {
                    key[index] -= 1;
                }
            }
            "r" => {
                index += 1;
            }
            "l" => {
                index -= 1;
            }
            "q" => {
                break;
            }
            _ => {
                println!("unknown command");
            }
        }
    }
}
