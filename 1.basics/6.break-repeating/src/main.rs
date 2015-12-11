// fn base64_value(byte: u8) -> u8 {
//     match byte {
//         b'A'...b'Z' => byte - b'A',
//         b'a'...b'z' => byte - b'a' + 26,
//         b'0'...b'9' => byte - b'0' + 52,
//         b'+' => 62,
//         b'/' => 63,
//         _ => panic!("invalid base64 character: {}", byte),
//     }
// }

// fn base64_decode(text: &[u8]) -> Vec<u8> {
//     let mut result: Vec<u8> = Vec::new();

//     if text.len() % 4 != 0 {
//         panic!("invalid base64");
//     }

//     for bytes in text.chunks(4) {
//         let mut accum: u32 = 0;
//         let mut equals = 0;

//         for i in (0..4) {
//             accum <<= 6;

//             if bytes[i] == b'=' {
//                 equals += 1;
//             } else {
//                 accum += base64_value(bytes[i]) as u32;
//             }
//         }

//         for i in (0..3 - equals) {
//             result.push(((accum >> (16 - 8 * i)) % 256) as u8);
//         }
//     }

//     result
// }

// fn test_data() -> Vec<u8> {
//     base64_decode(b"HUIfTQsPAh9PE048GmllH0kcDk4TAQsHThsBFkU2AB4BSWQgVB0dQzNTTmVS\
//     BgBHVBwNRU0HBAxTEjwMHghJGgkRTxRMIRpHKwAFHUdZEQQJAGQmB1MANxYG\
//     DBoXQR0BUlQwXwAgEwoFR08SSAhFTmU+Fgk4RQYFCBpGB08fWXh+amI2DB0P\
//     QQ1IBlUaGwAdQnQEHgFJGgkRAlJ6f0kASDoAGhNJGk9FSA8dDVMEOgFSGQEL\
//     QRMGAEwxX1NiFQYHCQdUCxdBFBZJeTM1CxsBBQ9GB08dTnhOSCdSBAcMRVhI\
//     CEEATyBUCHQLHRlJAgAOFlwAUjBpZR9JAgJUAAELB04CEFMBJhAVTQIHAh9P\
//     G054MGk2UgoBCVQGBwlTTgIQUwg7EAYFSQ8PEE87ADpfRyscSWQzT1QCEFMa\
//     TwUWEXQMBk0PAg4DQ1JMPU4ALwtJDQhOFw0VVB1PDhxFXigLTRkBEgcKVVN4\
//     Tk9iBgELR1MdDAAAFwoFHww6Ql5NLgFBIg4cSTRWQWI1Bk9HKn47CE8BGwFT\
//     QjcEBx4MThUcDgYHKxpUKhdJGQZZVCFFVwcDBVMHMUV4LAcKQR0JUlk3TwAm\
//     HQdJEwATARNFTg5JFwQ5C15NHQYEGk94dzBDADsdHE4UVBUaDE5JTwgHRTkA\
//     Umc6AUETCgYAN1xGYlUKDxJTEUgsAA0ABwcXOwlSGQELQQcbE0c9GioWGgwc\
//     AgcHSAtPTgsAABY9C1VNCAINGxgXRHgwaWUfSQcJABkRRU8ZAUkDDTUWF01j\
//     OgkRTxVJKlZJJwFJHQYADUgRSAsWSR8KIgBSAAxOABoLUlQwW1RiGxpOCEtU\
//     YiROCk8gUwY1C1IJCAACEU8QRSxORTBSHQYGTlQJC1lOBAAXRTpCUh0FDxhU\
//     ZXhzLFtHJ1JbTkoNVDEAQU4bARZFOwsXTRAPRlQYE042WwAuGxoaAk5UHAoA\
//     ZCYdVBZ0ChQLSQMYVAcXQTwaUy1SBQsTAAAAAAAMCggHRSQJExRJGgkGAAdH\
//     MBoqER1JJ0dDFQZFRhsBAlMMIEUHHUkPDxBPH0EzXwArBkkdCFUaDEVHAQAN\
//     U29lSEBAWk44G09fDXhxTi0RAk4ITlQbCk0LTx4cCjBFeCsGHEETAB1EeFZV\
//     IRlFTi4AGAEORU4CEFMXPBwfCBpOAAAdHUMxVVUxUmM9ElARGgZBAg4PAQQz\
//     DB4EGhoIFwoKUDFbTCsWBg0OTwEbRSonSARTBDpFFwsPCwIATxNOPBpUKhMd\
//     Th5PAUgGQQBPCxYRdG87TQoPD1QbE0s9GkFiFAUXR0cdGgkADwENUwg1DhdN\
//     AQsTVBgXVHYaKkg7TgNHTB0DAAA9DgQACjpFX0BJPQAZHB1OeE5PYjYMAg5M\
//     FQBFKjoHDAEAcxZSAwZOBREBC0k2HQxiKwYbR0MVBkVUHBZJBwp0DRMDDk5r\
//     NhoGACFVVWUeBU4MRREYRVQcFgAdQnQRHU0OCxVUAgsAK05ZLhdJZChWERpF\
//     QQALSRwTMRdeTRkcABcbG0M9Gk0jGQwdR1ARGgNFDRtJeSchEVIDBhpBHQlS\
//     WTdPBzAXSQ9HTBsJA0UcQUl5bw0KB0oFAkETCgYANlVXKhcbC0sAGgdFUAIO\
//     ChZJdAsdTR0HDBFDUk43GkcrAAUdRyonBwpOTkJEUyo8RR8USSkOEENSSDdX\
//     RSAdDRdLAA0HEAAeHQYRBDYJC00MDxVUZSFQOV1IJwYdB0dXHRwNAA9PGgMK\
//     OwtTTSoBDBFPHU54W04mUhoPHgAdHEQAZGU/OjV6RSQMBwcNGA5SaTtfADsX\
//     GUJHWREYSQAnSARTBjsIGwNOTgkVHRYANFNLJ1IIThVIHQYKAGQmBwcKLAwR\
//     DB0HDxNPAU94Q083UhoaBkcTDRcAAgYCFkU1RQUEBwFBfjwdAChPTikBSR0T\
//     TwRIEVIXBgcURTULFk0OBxMYTwFUN0oAIQAQBwkHVGIzQQAGBR8EdCwRCEkH\
//     ElQcF0w0U05lUggAAwANBxAAHgoGAwkxRRMfDE4DARYbTn8aKmUxCBsURVQf\
//     DVlOGwEWRTIXFwwCHUEVHRcAMlVDKRsHSUdMHQMAAC0dCAkcdCIeGAxOazkA\
//     BEk2HQAjHA1OAFIbBxNJAEhJBxctDBwKSRoOVBwbTj8aQS4dBwlHKjUECQAa\
//     BxscEDMNUhkBC0ETBxdULFUAJQAGARFJGk9FVAYGGlMNMRcXTRoBDxNPeG43\
//     TQA7HRxJFUVUCQhBFAoNUwctRQYFDE43PT9SUDdJUydcSWRtcwANFVAHAU5T\
//     FjtFGgwbCkEYBhlFeFsABRcbAwZOVCYEWgdPYyARNRcGAQwKQRYWUlQwXwAg\
//     ExoLFAAcARFUBwFOUwImCgcDDU5rIAcXUj0dU2IcBk4TUh0YFUkASEkcC3QI\
//     GwMMQkE9SB8AMk9TNlIOCxNUHQZCAAoAHh1FXjYCDBsFABkOBkk7FgALVQRO\
//     D0EaDwxOSU8dGgI8EVIBAAUEVA5SRjlUQTYbCk5teRsdRVQcDhkDADBFHwhJ\
//     AQ8XClJBNl4AC1IdBghVEwARABoHCAdFXjwdGEkDCBMHBgAwW1YnUgAaRyon\
//     B0VTGgoZUwE7EhxNCAAFVAMXTjwaTSdSEAESUlQNBFJOZU5LXHQMHE0EF0EA\
//     Bh9FeRp5LQdFTkAZREgMU04CEFMcMQQAQ0lkay0ABwcqXwA1FwgFAk4dBkIA\
//     CA4aB0l0PD1MSQ8PEE87ADtbTmIGDAILAB0cRSo3ABwBRTYKFhROHUETCgZU\
//     MVQHYhoGGksABwdJAB0ASTpFNwQcTRoDBBgDUkksGioRHUkKCE5THEVCC08E\
//     EgF0BBwJSQoOGkgGADpfADETDU5tBzcJEFMLTx0bAHQJCx8ADRJUDRdMN1RH\
//     YgYGTi5jMURFeQEaSRAEOkURDAUCQRkKUmQ5XgBIKwYbQFIRSBVJGgwBGgtz\
//     RRNNDwcVWE8BT3hJVCcCSQwGQx9IBE4KTwwdASEXF01jIgQATwZIPRpXKwYK\
//     BkdEGwsRTxxDSToGMUlSCQZOFRwKUkQ5VEMnUh0BR0MBGgAAZDwGUwY7CBdN\
//     HB5BFwMdUz0aQSwWSQoITlMcRUILTxoCEDUXF01jNw4BTwVBNlRBYhAIGhNM\
//     EUgIRU5CRFMkOhwGBAQLTVQOHFkvUkUwF0lkbXkbHUVUBgAcFA0gRQYFCBpB\
//     PU8FQSsaVycTAkJHYhsRSQAXABxUFzFFFggICkEDHR1OPxoqER1JDQhNEUgK\
//     TkJPDAUAJhwQAg0XQRUBFgArU04lUh0GDlNUGwpOCU9jeTY1HFJARE4xGA4L\
//     ACxSQTZSDxsJSw1ICFUdBgpTNjUcXk0OAUEDBxtUPRpCLQtFTgBPVB8NSRoK\
//     SREKLUUVAklkERgOCwAsUkE2Ug8bCUsNSAhVHQYKUyI7RQUFABoEVA0dWXQa\
//     Ry1SHgYOVBFIB08XQ0kUCnRvPgwQTgUbGBwAOVREYhAGAQBJEUgETgpPGR8E\
//     LUUGBQgaQRIaHEshGk03AQANR1QdBAkAFwAcUwE9AFxNY2QxGA4LACxSQTZS\
//     DxsJSw1ICFUdBgpTJjsIF00GAE1ULB1NPRpPLF5JAgJUVAUAAAYKCAFFXjUe\
//     DBBOFRwOBgA+T04pC0kDElMdC0VXBgYdFkU2CgtNEAEUVBwTWXhTVG5SGg8e\
//     AB0cRSo+AwgKRSANExlJCBQaBAsANU9TKxFJL0dMHRwRTAtPBRwQMAAATQcB\
//     FlRlIkw5QwA2GggaR0YBBg5ZTgIcAAw3SVIaAQcVEU8QTyEaYy0fDE4ITlhI\
//     Jk8DCkkcC3hFMQIEC0EbAVIqCFZBO1IdBgZUVA4QTgUWSR4QJwwRTWM=")
// }

// #[test]
// fn base64_test() {
//     assert_eq!(base64_decode(b"Y2F0cw=="), b"cats");
// }

// fn bit_count(mut x: u8) -> u32 {
//     let mut count = 0;

//     while x > 0 {
//         count += (x % 2) as u32;
//         x /= 2;
//     }

//     count
// }

// #[test]
// fn bit_count_test() {
//     assert_eq!(bit_count(0b_0101_1010), 4);
// }

// fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
//     a.iter().zip(b.iter()).fold(0, |sum, (x, y)| sum + bit_count(x ^ y))
// }

// #[test]
// fn hamming_distance_test() {
//     assert_eq!(hamming_distance(b"this is a test", b"wokka wokka!!!"), 37);
// }

// fn key_distance(key_size: usize, cipher: &[u8]) -> f64 {
//     let a = &cipher[0..key_size];
//     let b = &cipher[key_size..2*key_size];
    
//     (hamming_distance(a, b) as f64) / (key_size as f64)
// }

// fn sort_best_keys(cipher: &[u8]) -> Vec<usize> {
//     let mut key_sizes: Vec<usize> = (2..41).collect();
    
//     key_sizes.sort_by(|a, b| {
//         let a_dist = key_distance(*a, cipher);
//         let b_dist = key_distance(*b, cipher);
        
//         if a_dist < b_dist {
//             std::cmp::Ordering::Less
//         } else if a_dist > b_dist {
//             std::cmp::Ordering::Greater
//         } else {
//             std::cmp::Ordering::Equal
//         }
//     });
    
//     key_sizes
// }

// fn get_chunks(cipher: &[u8], key_size: usize) -> Vec<Vec<u8>> {
//     let mut result: Vec<Vec<u8>> = Vec::new();
    
//     for i in (0..key_size) {
//         result.push(Vec::new());
//     }
    
//     for chunk in cipher.chunks(key_size) {
//         for i in (0..chunk.len()) {
//             result[i].push(chunk[i]);
//         }
//     }
    
//     result
// }

// #[test]
// fn get_chunks_test() {
//     assert_eq!(get_chunks(b"hello", 2), [b"hlo".to_vec(), b"el".to_vec()]);
//     assert_eq!(get_chunks(b"hello", 3), [b"hl".to_vec(), b"eo".to_vec(), b"l".to_vec()]);
// }

// fn decrypt_chunk(chunk: &[u8], key: u8) -> Vec<u8> {
//     chunk.iter().map(|byte| byte ^ key).collect()
// }

// fn decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
//     cipher.iter().enumerate().map(|(i, item)| {
//         key[i % key.len()] ^ *item
//     }).collect()
// }

// fn score(letters: &Vec<u8>) -> i32 {
//     let mut total: i32 = 0;
    
//     for letter in letters.iter() {
//         match *letter {
//             b'a'...b'z' => total += 5,
//             b'A'...b'Z' => total += 5,
//             b'0'...b'9' => total += 2,
//             b' '  => total += 3,
//             b'\n' => total += 1,
//             b'\r' => total += 1,
//             b'!'  => total += 1,
//             b'.' => total += 1,
//             b'?' => total += 1,
//             0...20 => total -= 20,
//             _ => total -= 5
//         }
//     }
    
//     total
// }

// fn guess_chunk_key(chunk: &[u8]) -> u8 {
//     let mut best_score: i32 = 0;
//     let mut best_key: u8 = 0;
//     let mut best_text: Vec<u8> = Vec::new();

//     for key in (0..255) {
//         let text = decrypt_chunk(&chunk, key);
//         let score = score(&text);
        
//         // println!("best score = {}", score);
//         // println!("best key = {}", key);
//         // println!("best text = {}", String::from_utf8(text.clone()).unwrap());

//         if score > best_score {
//             best_score = score;
//             best_key = key;
//             best_text = text;
//         }
//     }

//     // println!("best text = {}", String::from_utf8(best_text.clone()).unwrap());
//     best_key
// }

// fn main() {
//     let cipher = test_data();    
    
//     let key_sizes = sort_best_keys(&cipher);
    
//     for key_size in key_sizes.iter().take(10) {
//         println!("Trying key size {}.", key_size);
       
//         let chunks = get_chunks(&cipher, *key_size);
       
//         for chunk in chunks.iter() {
//             let key = guess_chunk_key(chunk);
//             println!("{}", std::char::from_u32(key as u32).unwrap());
//         } 
//     }
    
//     // let chunks = get_chunks(&cipher, key_size);
    
//     // for chunk in chunks.iter() {
//     //     let key = guess_chunk_key(chunk);
//     //     println!("{}", std::char::from_u32(key as u32).unwrap());
//     // }
// }

// extern crate pals;

mod pals;

fn main() {
    println!("{}", pals::hello());
    println!("cats in hex is {:?}", pals::hex::decode("cats").unwrap());
}
