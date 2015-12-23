#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn test_data() -> Vec<Vec<u8>> {
    let ciphers = ["SSdtIHJhdGVkICJSIi4uLnRoaXMgaXMgYSB3YXJuaW5nLCB5YSBiZXR0ZXIgdm9pZCAvIFBvZXRzI\
                    GFyZSBwYXJhbm9pZCwgREoncyBELXN0cm95ZWQ=",
                   "Q3V6IEkgY2FtZSBiYWNrIHRvIGF0dGFjayBvdGhlcnMgaW4gc3BpdGUtIC8gU3RyaWtlIGxpa2Ugb\
                    GlnaHRuaW4nLCBJdCdzIHF1aXRlIGZyaWdodGVuaW4nIQ==",
                   "QnV0IGRvbid0IGJlIGFmcmFpZCBpbiB0aGUgZGFyaywgaW4gYSBwYXJrIC8gTm90IGEgc2NyZWFtI\
                    G9yIGEgY3J5LCBvciBhIGJhcmssIG1vcmUgbGlrZSBhIHNwYXJrOw==",
                   "WWEgdHJlbWJsZSBsaWtlIGEgYWxjb2hvbGljLCBtdXNjbGVzIHRpZ2h0ZW4gdXAgLyBXaGF0J3Mgd\
                    GhhdCwgbGlnaHRlbiB1cCEgWW91IHNlZSBhIHNpZ2h0IGJ1dA==",
                   "U3VkZGVubHkgeW91IGZlZWwgbGlrZSB5b3VyIGluIGEgaG9ycm9yIGZsaWNrIC8gWW91IGdyYWIge\
                    W91ciBoZWFydCB0aGVuIHdpc2ggZm9yIHRvbW9ycm93IHF1aWNrIQ==",
                   "TXVzaWMncyB0aGUgY2x1ZSwgd2hlbiBJIGNvbWUgeW91ciB3YXJuZWQgLyBBcG9jYWx5cHNlIE5vd\
                    ywgd2hlbiBJJ20gZG9uZSwgeWEgZ29uZSE=",
                   "SGF2ZW4ndCB5b3UgZXZlciBoZWFyZCBvZiBhIE1DLW11cmRlcmVyPyAvIFRoaXMgaXMgdGhlIGRlY\
                    XRoIHBlbmFsdHksYW5kIEknbSBzZXJ2aW4nIGE=",
                   "RGVhdGggd2lzaCwgc28gY29tZSBvbiwgc3RlcCB0byB0aGlzIC8gSHlzdGVyaWNhbCBpZGVhIGZvc\
                    iBhIGx5cmljYWwgcHJvZmVzc2lvbmlzdCE=",
                   "RnJpZGF5IHRoZSB0aGlydGVlbnRoLCB3YWxraW5nIGRvd24gRWxtIFN0cmVldCAvIFlvdSBjb21lI\
                    GluIG15IHJlYWxtIHlhIGdldCBiZWF0IQ==",
                   "VGhpcyBpcyBvZmYgbGltaXRzLCBzbyB5b3VyIHZpc2lvbnMgYXJlIGJsdXJyeSAvIEFsbCB5YSBzZ\
                    WUgaXMgdGhlIG1ldGVycyBhdCBhIHZvbHVtZQ==",
                   "VGVycm9yIGluIHRoZSBzdHlsZXMsIG5ldmVyIGVycm9yLWZpbGVzIC8gSW5kZWVkIEknbSBrbm93b\
                    i15b3VyIGV4aWxlZCE=",
                   "Rm9yIHRob3NlIHRoYXQgb3Bwb3NlIHRvIGJlIGxldmVsIG9yIG5leHQgdG8gdGhpcyAvIEkgYWluJ\
                    3QgYSBkZXZpbCBhbmQgdGhpcyBhaW4ndCB0aGUgRXhvcmNpc3Qh",
                   "V29yc2UgdGhhbiBhIG5pZ2h0bWFyZSwgeW91IGRvbid0IGhhdmUgdG8gc2xlZXAgYSB3aW5rIC8gV\
                    GhlIHBhaW4ncyBhIG1pZ3JhaW5lIGV2ZXJ5IHRpbWUgeWEgdGhpbms=",
                   "Rmxhc2hiYWNrcyBpbnRlcmZlcmUsIHlhIHN0YXJ0IHRvIGhlYXI6IC8gVGhlIFItQS1LLUktTSBpb\
                    iB5b3VyIGVhcjs=",
                   "VGhlbiB0aGUgYmVhdCBpcyBoeXN0ZXJpY2FsIC8gVGhhdCBtYWtlcyBFcmljIGdvIGdldCBhIGF4I\
                    GFuZCBjaG9wcyB0aGUgd2Fjaw==",
                   "U29vbiB0aGUgbHlyaWNhbCBmb3JtYXQgaXMgc3VwZXJpb3IgLyBGYWNlcyBvZiBkZWF0aCByZW1ha\
                    W4=",
                   "TUMncyBkZWNheWluZywgY3V6IHRoZXkgbmV2ZXIgc3RheWVkIC8gVGhlIHNjZW5lIG9mIGEgY3Jpb\
                    WUgZXZlcnkgbmlnaHQgYXQgdGhlIHNob3c=",
                   "VGhlIGZpZW5kIG9mIGEgcmh5bWUgb24gdGhlIG1pYyB0aGF0IHlvdSBrbm93IC8gSXQncyBvbmx5I\
                    G9uZSBjYXBhYmxlLCBicmVha3MtdGhlIHVuYnJlYWthYmxl",
                   "TWVsb2RpZXMtdW5tYWthYmxlLCBwYXR0ZXJuLXVuZXNjYXBhYmxlIC8gQSBob3JuIGlmIHdhbnQgd\
                    GhlIHN0eWxlIEkgcG9zc2Vz",
                   "SSBibGVzcyB0aGUgY2hpbGQsIHRoZSBlYXJ0aCwgdGhlIGdvZHMgYW5kIGJvbWIgdGhlIHJlc3QgL\
                    yBGb3IgdGhvc2UgdGhhdCBlbnZ5IGEgTUMgaXQgY2FuIGJl",
                   "SGF6YXJkb3VzIHRvIHlvdXIgaGVhbHRoIHNvIGJlIGZyaWVuZGx5IC8gQSBtYXR0ZXIgb2YgbGlmZ\
                    SBhbmQgZGVhdGgsIGp1c3QgbGlrZSBhIGV0Y2gtYS1za2V0Y2g=",
                   "U2hha2UgJ3RpbGwgeW91ciBjbGVhciwgbWFrZSBpdCBkaXNhcHBlYXIsIG1ha2UgdGhlIG5leHQgL\
                    yBBZnRlciB0aGUgY2VyZW1vbnksIGxldCB0aGUgcmh5bWUgcmVzdCBpbiBwZWFjZQ==",
                   "SWYgbm90LCBteSBzb3VsJ2xsIHJlbGVhc2UhIC8gVGhlIHNjZW5lIGlzIHJlY3JlYXRlZCwgcmVpb\
                    mNhcm5hdGVkLCB1cGRhdGVkLCBJJ20gZ2xhZCB5b3UgbWFkZSBpdA==",
                   "Q3V6IHlvdXIgYWJvdXQgdG8gc2VlIGEgZGlzYXN0cm91cyBzaWdodCAvIEEgcGVyZm9ybWFuY2Ugb\
                    mV2ZXIgYWdhaW4gcGVyZm9ybWVkIG9uIGEgbWljOg==",
                   "THlyaWNzIG9mIGZ1cnkhIEEgZmVhcmlmaWVkIGZyZWVzdHlsZSEgLyBUaGUgIlIiIGlzIGluIHRoZ\
                    SBob3VzZS10b28gbXVjaCB0ZW5zaW9uIQ==",
                   "TWFrZSBzdXJlIHRoZSBzeXN0ZW0ncyBsb3VkIHdoZW4gSSBtZW50aW9uIC8gUGhyYXNlcyB0aGF0J\
                    3MgZmVhcnNvbWU=",
                   "WW91IHdhbnQgdG8gaGVhciBzb21lIHNvdW5kcyB0aGF0IG5vdCBvbmx5IHBvdW5kcyBidXQgcGxlY\
                    XNlIHlvdXIgZWFyZHJ1bXM7IC8gSSBzaXQgYmFjayBhbmQgb2JzZXJ2ZSB0aGUgd2hvbGUgc2Nlbm\
                    VyeQ==",
                   "VGhlbiBub25jaGFsYW50bHkgdGVsbCB5b3Ugd2hhdCBpdCBtZWFuIHRvIG1lIC8gU3RyaWN0bHkgY\
                    nVzaW5lc3MgSSdtIHF1aWNrbHkgaW4gdGhpcyBtb29k",
                   "QW5kIEkgZG9uJ3QgY2FyZSBpZiB0aGUgd2hvbGUgY3Jvd2QncyBhIHdpdG5lc3MhIC8gSSdtIGEgd\
                    GVhciB5b3UgYXBhcnQgYnV0IEknbSBhIHNwYXJlIHlvdSBhIGhlYXJ0",
                   "UHJvZ3JhbSBpbnRvIHRoZSBzcGVlZCBvZiB0aGUgcmh5bWUsIHByZXBhcmUgdG8gc3RhcnQgLyBSa\
                    Hl0aG0ncyBvdXQgb2YgdGhlIHJhZGl1cywgaW5zYW5lIGFzIHRoZSBjcmF6aWVzdA==",
                   "TXVzaWNhbCBtYWRuZXNzIE1DIGV2ZXIgbWFkZSwgc2VlIGl0J3MgLyBOb3cgYW4gZW1lcmdlbmN5L\
                    CBvcGVuLWhlYXJ0IHN1cmdlcnk=",
                   "T3BlbiB5b3VyIG1pbmQsIHlvdSB3aWxsIGZpbmQgZXZlcnkgd29yZCdsbCBiZSAvIEZ1cmllciB0a\
                    GFuIGV2ZXIsIEkgcmVtYWluIHRoZSBmdXJ0dXJl",
                   "QmF0dGxlJ3MgdGVtcHRpbmcuLi53aGF0ZXZlciBzdWl0cyB5YSEgLyBGb3Igd29yZHMgdGhlIHNlb\
                    nRlbmNlLCB0aGVyZSdzIG5vIHJlc2VtYmxhbmNl",
                   "WW91IHRoaW5rIHlvdSdyZSBydWZmZXIsIHRoZW4gc3VmZmVyIHRoZSBjb25zZXF1ZW5jZXMhIC8gS\
                    SdtIG5ldmVyIGR5aW5nLXRlcnJpZnlpbmcgcmVzdWx0cw==",
                   "SSB3YWtlIHlhIHdpdGggaHVuZHJlZHMgb2YgdGhvdXNhbmRzIG9mIHZvbHRzIC8gTWljLXRvLW1vd\
                    XRoIHJlc3VzY2l0YXRpb24sIHJoeXRobSB3aXRoIHJhZGlhdGlvbg==",
                   "Tm92b2NhaW4gZWFzZSB0aGUgcGFpbiBpdCBtaWdodCBzYXZlIGhpbSAvIElmIG5vdCwgRXJpYyBCL\
                    idzIHRoZSBqdWRnZSwgdGhlIGNyb3dkJ3MgdGhlIGp1cnk=",
                   "WW8gUmFraW0sIHdoYXQncyB1cD8gLyBZbywgSSdtIGRvaW5nIHRoZSBrbm93bGVkZ2UsIEUuLCBtY\
                    W4gSSdtIHRyeWluZyB0byBnZXQgcGFpZCBpbiBmdWxs",
                   "V2VsbCwgY2hlY2sgdGhpcyBvdXQsIHNpbmNlIE5vcmJ5IFdhbHRlcnMgaXMgb3VyIGFnZW5jeSwgc\
                    mlnaHQ/IC8gVHJ1ZQ==",
                   "S2FyYSBMZXdpcyBpcyBvdXIgYWdlbnQsIHdvcmQgdXAgLyBaYWtpYSBhbmQgNHRoIGFuZCBCcm9hZ\
                    HdheSBpcyBvdXIgcmVjb3JkIGNvbXBhbnksIGluZGVlZA==",
                   "T2theSwgc28gd2hvIHdlIHJvbGxpbicgd2l0aCB0aGVuPyBXZSByb2xsaW4nIHdpdGggUnVzaCAvI\
                    E9mIFJ1c2h0b3duIE1hbmFnZW1lbnQ=",
                   "Q2hlY2sgdGhpcyBvdXQsIHNpbmNlIHdlIHRhbGtpbmcgb3ZlciAvIFRoaXMgZGVmIGJlYXQgcmlna\
                    HQgaGVyZSB0aGF0IEkgcHV0IHRvZ2V0aGVy",
                   "SSB3YW5uYSBoZWFyIHNvbWUgb2YgdGhlbSBkZWYgcmh5bWVzLCB5b3Uga25vdyB3aGF0IEknbSBzY\
                    Xlpbic/IC8gQW5kIHRvZ2V0aGVyLCB3ZSBjYW4gZ2V0IHBhaWQgaW4gZnVsbA==",
                   "VGhpbmtpbicgb2YgYSBtYXN0ZXIgcGxhbiAvICdDdXogYWluJ3QgbnV0aGluJyBidXQgc3dlYXQga\
                    W5zaWRlIG15IGhhbmQ=",
                   "U28gSSBkaWcgaW50byBteSBwb2NrZXQsIGFsbCBteSBtb25leSBpcyBzcGVudCAvIFNvIEkgZGlnI\
                    GRlZXBlciBidXQgc3RpbGwgY29taW4nIHVwIHdpdGggbGludA==",
                   "U28gSSBzdGFydCBteSBtaXNzaW9uLCBsZWF2ZSBteSByZXNpZGVuY2UgLyBUaGlua2luJyBob3cgY\
                    291bGQgSSBnZXQgc29tZSBkZWFkIHByZXNpZGVudHM=",
                   "SSBuZWVkIG1vbmV5LCBJIHVzZWQgdG8gYmUgYSBzdGljay11cCBraWQgLyBTbyBJIHRoaW5rIG9mI\
                    GFsbCB0aGUgZGV2aW91cyB0aGluZ3MgSSBkaWQ=",
                   "SSB1c2VkIHRvIHJvbGwgdXAsIHRoaXMgaXMgYSBob2xkIHVwLCBhaW4ndCBudXRoaW4nIGZ1bm55I\
                    C8gU3RvcCBzbWlsaW5nLCBiZSBzdGlsbCwgZG9uJ3QgbnV0aGluJyBtb3ZlIGJ1dCB0aGUgbW9uZX\
                    k=",
                   "QnV0IG5vdyBJIGxlYXJuZWQgdG8gZWFybiAnY3V6IEknbSByaWdodGVvdXMgLyBJIGZlZWwgZ3JlY\
                    XQsIHNvIG1heWJlIEkgbWlnaHQganVzdA==",
                   "U2VhcmNoIGZvciBhIG5pbmUgdG8gZml2ZSwgaWYgSSBzdHJpdmUgLyBUaGVuIG1heWJlIEknbGwgc\
                    3RheSBhbGl2ZQ==",
                   "U28gSSB3YWxrIHVwIHRoZSBzdHJlZXQgd2hpc3RsaW4nIHRoaXMgLyBGZWVsaW4nIG91dCBvZiBwb\
                    GFjZSAnY3V6LCBtYW4sIGRvIEkgbWlzcw==",
                   "QSBwZW4gYW5kIGEgcGFwZXIsIGEgc3RlcmVvLCBhIHRhcGUgb2YgLyBNZSBhbmQgRXJpYyBCLCBhb\
                    mQgYSBuaWNlIGJpZyBwbGF0ZSBvZg==",
                   "RmlzaCwgd2hpY2ggaXMgbXkgZmF2b3JpdGUgZGlzaCAvIEJ1dCB3aXRob3V0IG5vIG1vbmV5IGl0J\
                    3Mgc3RpbGwgYSB3aXNo",
                   "J0N1eiBJIGRvbid0IGxpa2UgdG8gZHJlYW0gYWJvdXQgZ2V0dGluJyBwYWlkIC8gU28gSSBkaWcga\
                    W50byB0aGUgYm9va3Mgb2YgdGhlIHJoeW1lcyB0aGF0IEkgbWFkZQ==",
                   "U28gbm93IHRvIHRlc3QgdG8gc2VlIGlmIEkgZ290IHB1bGwgLyBIaXQgdGhlIHN0dWRpbywgJ2N1e\
                    iBJJ20gcGFpZCBpbiBmdWxs",
                   "UmFraW0sIGNoZWNrIHRoaXMgb3V0LCB5byAvIFlvdSBnbyB0byB5b3VyIGdpcmwgaG91c2UgYW5kI\
                    EknbGwgZ28gdG8gbWluZQ==",
                   "J0NhdXNlIG15IGdpcmwgaXMgZGVmaW5pdGVseSBtYWQgLyAnQ2F1c2UgaXQgdG9vayB1cyB0b28gb\
                    G9uZyB0byBkbyB0aGlzIGFsYnVt",
                   "WW8sIEkgaGVhciB3aGF0IHlvdSdyZSBzYXlpbmcgLyBTbyBsZXQncyBqdXN0IHB1bXAgdGhlIG11c\
                    2ljIHVw",
                   "QW5kIGNvdW50IG91ciBtb25leSAvIFlvLCB3ZWxsIGNoZWNrIHRoaXMgb3V0LCB5byBFbGk=",
                   "VHVybiBkb3duIHRoZSBiYXNzIGRvd24gLyBBbmQgbGV0IHRoZSBiZWF0IGp1c3Qga2VlcCBvbiByb\
                    2NraW4n",
                   "QW5kIHdlIG91dHRhIGhlcmUgLyBZbywgd2hhdCBoYXBwZW5lZCB0byBwZWFjZT8gLyBQZWFjZQ=="];

    ciphers.iter().map(|cipher| pals::base64::decode(cipher).unwrap()).collect()
}

fn common_length(ciphers: &Vec<Vec<u8>>) -> usize {
    let mut length = ciphers[0].len();

    for cipher in ciphers {
        length = std::cmp::max(length, cipher.len());
    }

    length
}

fn score(letter: u8) -> i32 {
    match letter {
        b'a'...b'z' => 10,
        b'A'...b'Z' => 10,
        b'0'...b'9' => 1,
        b' ' => 5,
        b'\'' => 1,
        b'"' => 1,
        b'\n' => 1,
        b'\r' => 1,
        b'!' => 1,
        b'.' => 1,
        b'?' => 1,
        0...20 => -20,
        128...255 => -20,
        _ => -5,
    }
}

fn score_index(ciphers: &Vec<Vec<u8>>, index: usize, guess: u8) -> i32 {
    let mut total: i32 = 0;

    for cipher in ciphers {
        if cipher.len() > index {
            total += score(cipher[index] ^ guess);
        }
    }

    total
}

fn guess_key_byte(ciphers: &Vec<Vec<u8>>, index: usize) -> u8 {
    let mut best_score = 0;
    let mut best_guess = 0;

    for i in 0..256 {
        let guess = i as u8;
        let score = score_index(ciphers, index, guess);

        if score > best_score {
            best_score = score;
            best_guess = guess;
        }
    }

    best_guess
}

fn guess_key(ciphers: &Vec<Vec<u8>>, length: usize) -> Vec<u8> {
    let mut key = Vec::new();

    for i in 0..length {
        key.push(guess_key_byte(ciphers, i));
    }

    key
}

fn printable(data: &[u8], key: &[u8]) -> String {
    let mut result = Vec::new();

    for (index, encrypted) in data.iter().enumerate() {
        if index >= key.len() {
            break;
        }

        let byte = encrypted ^ key[index];

        if byte >= 32 && byte <= 127 {
            result.push(byte);
        } else {
            result.push(b'.');
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    let ciphers = test_data();
    let length = common_length(&ciphers);
    let key = guess_key(&ciphers, length);

    for cipher in ciphers {
        println!("{}", printable(&cipher, &key));
    }
}
