const I0: u32 = 0x67452301;
const I1: u32 = 0xefcdab89;
const I2: u32 = 0x98badcfe;
const I3: u32 = 0x10325476;
const K2: u32 = 0x5A827999;
const K3: u32 = 0x6ED9EBA1;

pub struct Hasher {
    state: [u32; 4],
    data_len: usize,
    buffer: [u32; 16],
    buffer_len: usize,
}

fn rotate_left(num: u32, amount: u8) -> u32 {
    (num << amount) | (num >> (32 - amount))
}

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

impl Hasher {
    pub fn new() -> Hasher {
        Hasher {
            state: [I0, I1, I2, I3],
            data_len: 0,
            buffer: [0; 16],
            buffer_len: 0,
        }
    }

    pub fn from(state: &[u8; 16], data_len: usize) -> Hasher {
        let mut hasher = Hasher {
            state: [I0, I1, I2, I3],
            data_len: data_len,
            buffer: [0; 16],
            buffer_len: 0,
        };

        for (i, byte) in state.iter().enumerate() {
            hasher.state[i / 4] = ((*byte as u32) << 24) | (hasher.state[i / 4] >> 8);
        }

        hasher
    }

    fn round1(&self, w: u32, x: u32, y: u32, z: u32, k: usize, s: u8) -> u32 {
        let temp = w.wrapping_add(f(x, y, z)).wrapping_add(self.buffer[k]);
        rotate_left(temp, s)
    }

    fn round2(&self, w: u32, x: u32, y: u32, z: u32, k: usize, s: u8) -> u32 {
        let temp = w.wrapping_add(g(x, y, z)).wrapping_add(self.buffer[k]).wrapping_add(K2);
        rotate_left(temp, s)
    }

    fn round3(&self, w: u32, x: u32, y: u32, z: u32, k: usize, s: u8) -> u32 {
        let temp = w.wrapping_add(h(x, y, z)).wrapping_add(self.buffer[k]).wrapping_add(K3);
        rotate_left(temp, s)
    }

    fn hash_block(&mut self) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        a = self.round1(a, b, c, d, 0, 3);
        d = self.round1(d, a, b, c, 1, 7);
        c = self.round1(c, d, a, b, 2, 11);
        b = self.round1(b, c, d, a, 3, 19);
        a = self.round1(a, b, c, d, 4, 3);
        d = self.round1(d, a, b, c, 5, 7);
        c = self.round1(c, d, a, b, 6, 11);
        b = self.round1(b, c, d, a, 7, 19);
        a = self.round1(a, b, c, d, 8, 3);
        d = self.round1(d, a, b, c, 9, 7);
        c = self.round1(c, d, a, b, 10, 11);
        b = self.round1(b, c, d, a, 11, 19);
        a = self.round1(a, b, c, d, 12, 3);
        d = self.round1(d, a, b, c, 13, 7);
        c = self.round1(c, d, a, b, 14, 11);
        b = self.round1(b, c, d, a, 15, 19);

        a = self.round2(a, b, c, d, 0, 3);
        d = self.round2(d, a, b, c, 4, 5);
        c = self.round2(c, d, a, b, 8, 9);
        b = self.round2(b, c, d, a, 12, 13);
        a = self.round2(a, b, c, d, 1, 3);
        d = self.round2(d, a, b, c, 5, 5);
        c = self.round2(c, d, a, b, 9, 9);
        b = self.round2(b, c, d, a, 13, 13);
        a = self.round2(a, b, c, d, 2, 3);
        d = self.round2(d, a, b, c, 6, 5);
        c = self.round2(c, d, a, b, 10, 9);
        b = self.round2(b, c, d, a, 14, 13);
        a = self.round2(a, b, c, d, 3, 3);
        d = self.round2(d, a, b, c, 7, 5);
        c = self.round2(c, d, a, b, 11, 9);
        b = self.round2(b, c, d, a, 15, 13);

        a = self.round3(a, b, c, d, 0, 3);
        d = self.round3(d, a, b, c, 8, 9);
        c = self.round3(c, d, a, b, 4, 11);
        b = self.round3(b, c, d, a, 12, 15);
        a = self.round3(a, b, c, d, 2, 3);
        d = self.round3(d, a, b, c, 10, 9);
        c = self.round3(c, d, a, b, 6, 11);
        b = self.round3(b, c, d, a, 14, 15);
        a = self.round3(a, b, c, d, 1, 3);
        d = self.round3(d, a, b, c, 9, 9);
        c = self.round3(c, d, a, b, 5, 11);
        b = self.round3(b, c, d, a, 13, 15);
        a = self.round3(a, b, c, d, 3, 3);
        d = self.round3(d, a, b, c, 11, 9);
        c = self.round3(c, d, a, b, 7, 11);
        b = self.round3(b, c, d, a, 15, 15);

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn add_byte(&mut self, value: u8) {
        let bucket = self.buffer_len / 4;

        self.buffer[bucket] = ((value as u32) << 24) | (self.buffer[bucket] >> 8);
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
            self.add_byte(((bit_len >> (8 * i)) % 256) as u8);
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        for byte in data.iter() {
            self.add_byte(*byte);
        }

        self.data_len += data.len();
    }

    pub fn end(&mut self) -> [u8; 16] {
        self.pad();

        let mut result = [0; 16];

        for (bucket, value) in self.state.iter().enumerate() {
            for byte in 0..4 {
                result[4 * bucket + byte] = ((value >> (8 * byte)) % 256) as u8;
            }
        }

        result
    }
}

pub fn hash(data: &[u8]) -> [u8; 16] {
    let mut hasher = Hasher::new();
    hasher.write(data);
    hasher.end()
}
