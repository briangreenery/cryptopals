pub mod hex;
pub mod base64;
pub mod aes;

pub mod mt19937;
pub use self::mt19937::MT19937;

mod hamming;
pub use self::hamming::hamming_distance;

pub mod sha1;
pub mod md4;

pub mod bn;
pub use self::bn::BigNum;

pub mod dsa;
pub use self::dsa::DSA;

pub mod cookie;

pub fn printable(bytes: &[u8]) -> String {
    let sanitized = bytes.iter()
                         .map(|byte| {
                             match *byte {
                                 32...126 => *byte,
                                 _ => b'.',
                             }
                         })
                         .collect();

    String::from_utf8(sanitized).unwrap()
}
