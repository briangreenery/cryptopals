use std::io::Write;

fn hex_from_num(num: u8) -> u8 {
    match num {
        0...9 => num + 48,
        _ => num + 87,
    }
}

struct ReapeatingXOR<R> {
    inner: R,
    index: usize,
    key: Vec<u8>,
}

impl<R: std::io::Read> ReapeatingXOR<R> {
    fn new(inner: R, key: &str) -> ReapeatingXOR<R> {
        ReapeatingXOR {
            inner: inner,
            index: 0,
            key: key.as_bytes().to_vec(),
        }
    }
}

impl<R: std::io::Read> Iterator for ReapeatingXOR<R> {
    type Item = std::io::Result<u8>;

    fn next(&mut self) -> Option<std::io::Result<u8>> {
        let mut buf = [0];

        match self.inner.read(&mut buf) {
            Ok(0) => None,
            Ok(..) => {
                let encrypted = self.key[self.index] ^ buf[0];
                self.index = (self.index + 1) % self.key.len();
                Some(Ok(encrypted))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let xor = ReapeatingXOR::new(stdin, "ICE");

    for byte in xor {
        let value = byte.unwrap();
        let hex = [hex_from_num(value / 16), hex_from_num(value % 16)];
        stdout.write(&hex).unwrap();
        stdout.flush();
    }
}
