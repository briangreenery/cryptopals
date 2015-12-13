pub mod hex;
pub mod base64;
pub mod aes;

mod hamming;
pub use self::hamming::hamming_distance;
