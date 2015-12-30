struct Hasher {
    h: [u32; 5],
}

fn rotate_left(num: u32, amount: u8) -> u32 {
    (num << amount) | (num >> (32 - amount))
}

impl Hasher {
    fn new() -> Hasher {
        Hasher { h: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0] }
    }

    fn hash(&mut self, data: &[u8]) -> Vec<u8> {
        let mut padded = Vec::new();

        padded.extend(data);
        padded.push(0x80);

        while (padded.len() + 8) % 64 != 0 {
            padded.push(0x00);
        }

        let len: u64 = 8 * (data.len() as u64);

        for i in 0..8 {
            padded.push(((len >> (56 - (8 * i))) & 0xFF) as u8);
        }

        for chunk in padded.chunks(64) {
            self.hash_block(chunk);
        }

        let mut result = Vec::new();

        for chunk in self.h.iter() {
            for i in 0..4 {
                result.push(((*chunk >> (24 - (8 * i))) & 0xFF) as u8);
            }
        }

        result
    }

    fn hash_block(&mut self, block: &[u8]) {
        let mut w: [u32; 80] = [0; 80];

        for (i, chunk) in block.chunks(4).enumerate() {
            w[i] = ((chunk[0] as u32) << 24) | ((chunk[1] as u32) << 16) |
                   ((chunk[2] as u32) << 8) | (chunk[3] as u32);
        }

        for i in 16..80 {
            w[i] = rotate_left(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];

        for i in 0..80 {
            let mut f = 0;
            let mut k = 0;

            if i < 20 {
                f = (b & c) | (!b & d);
                k = 0x5A827999;
            } else if 20 <= i && i < 40 {
                f = b ^ c ^ d;
                k = 0x6ED9EBA1;
            } else if 40 <= i && i < 60 {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8F1BBCDC;
            } else if 60 <= i {
                f = b ^ c ^ d;
                k = 0xCA62C1D6;
            }

            let temp = rotate_left(a, 5)
                           .wrapping_add(f)
                           .wrapping_add(e)
                           .wrapping_add(k)
                           .wrapping_add(w[i]);
            e = d;
            d = c;
            c = rotate_left(b, 30);
            b = a;
            a = temp;
        }

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
    }
}

pub fn hash(data: &[u8]) -> Vec<u8> {
    Hasher::new().hash(data)
}
