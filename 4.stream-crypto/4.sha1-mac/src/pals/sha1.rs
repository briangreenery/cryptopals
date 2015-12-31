const I0: u32 = 0x67452301;
const I1: u32 = 0xEFCDAB89;
const I2: u32 = 0x98BADCFE;
const I3: u32 = 0x10325476;
const I4: u32 = 0xC3D2E1F0;
const K0: u32 = 0x5A827999;
const K1: u32 = 0x6ED9EBA1;
const K2: u32 = 0x8F1BBCDC;
const K3: u32 = 0xCA62C1D6;

pub struct Hasher {
    state: [u32; 5],
    data_len: usize,
    buffer: [u32; 16],
    buffer_len: usize,
}

fn rotate_left(num: u32, amount: u8) -> u32 {
    (num << amount) | (num >> (32 - amount))
}

impl Hasher {
    pub fn new() -> Hasher {
        Hasher {
            state: [I0, I1, I2, I3, I4],
            data_len: 0,
            buffer: [0; 16],
            buffer_len: 0,
        }
    }

    fn hash_block(&mut self) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];

        for i in 0..80 {
            if i >= 16 {
                self.buffer[i % 16] = rotate_left(self.buffer[(i - 3) % 16] ^
                                                  self.buffer[(i - 8) % 16] ^
                                                  self.buffer[(i - 14) % 16] ^
                                                  self.buffer[i % 16],
                                                  1);
            }

            let combined = match i {
                0...19 => K0.wrapping_add((b & c) | (!b & d)),
                20...39 => K1.wrapping_add(b ^ c ^ d),
                40...59 => K2.wrapping_add((b & c) | (b & d) | (c & d)),
                _ => K3.wrapping_add(b ^ c ^ d),
            };

            let rotated = rotate_left(a, 5).wrapping_add(e);

            e = d;
            d = c;
            c = rotate_left(b, 30);
            b = a;
            a = self.buffer[i % 16].wrapping_add(combined).wrapping_add(rotated);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
    }

    fn add_byte(&mut self, value: u8) {
        let bucket = self.buffer_len / 4;

        self.buffer[bucket] = (self.buffer[bucket] << 8) | (value as u32);
        self.buffer_len += 1;

        if self.buffer_len == 64 {
            self.hash_block();
            self.buffer_len = 0;
        }
    }

    fn pad(&mut self) {
        self.add_byte(0x80);

        while self.buffer_len != 56 {
            self.add_byte(0x00);
        }

        let bit_len = (8 * self.data_len) as u64;

        for i in 0..8 {
            self.add_byte(((bit_len >> (56 - (8 * i))) % 256) as u8);
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        for byte in data.iter() {
            self.add_byte(*byte);
        }

        self.data_len += data.len();
    }

    pub fn end(&mut self) -> [u8; 20] {
        self.pad();

        let mut result = [0; 20];

        for (bucket, value) in self.state.iter().enumerate() {
            for byte in 0..4 {
                result[4 * bucket + byte] = (value >> (24 - (8 * byte))) as u8;
            }
        }

        result
    }
}

pub fn hash(data: &[u8]) -> [u8; 20] {
    let mut hasher = Hasher::new();
    hasher.write(data);
    hasher.end()
}
