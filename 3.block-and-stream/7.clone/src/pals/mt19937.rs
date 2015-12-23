const N: usize = 624;
const F: u32 = 1812433253;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const A: u32 = 0x9908B0DF;
const M: usize = 397;
const LOWER_MASK: u32 = 0x7fffffff;
const UPPER_MASK: u32 = 0x80000000;

pub struct MT19937 {
    state: [u32; N],
    index: usize,
}

impl MT19937 {
    pub fn new(seed: u32) -> MT19937 {
        let mut mt = MT19937 {
            state: [0; N],
            index: N,
        };

        mt.state[0] = seed;
        for i in 1..N {
            mt.state[i] = F.wrapping_mul(mt.state[i - 1] ^ (mt.state[i - 1] >> 30))
                           .wrapping_add(i as u32);
        }

        mt
    }

    pub fn copy(stuf: &[u32]) -> MT19937 {
        let mut mt = MT19937 {
            state: [0; N],
            index: N,
        };

        for i in 0..N {
            mt.state[i] = untemper(stuf[i]);
        }

        mt
    }

    pub fn gen(&mut self) -> u32 {
        if self.index >= N {
            self.twist();
            self.index = 0;
        }

        let mut y = self.state[self.index];
        self.index += 1;

        y ^= (y >> U) & D;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;
        y
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x = (self.state[i] & UPPER_MASK).wrapping_add(self.state[(i + 1) % N] & LOWER_MASK);

            let x_a = if x % 2 != 0 {
                (x >> 1) ^ A
            } else {
                (x >> 1)
            };

            self.state[i] = self.state[(i + M) % N] ^ x_a;
        }
    }
}

pub fn temper(value: u32) -> u32 {
    let mut y = value;

    y ^= (y >> U) & D;
    y ^= (y << S) & B;
    y ^= (y << T) & C;
    y ^= y >> L;

    y
}

pub fn untemper(value: u32) -> u32 {
    let mut y = value;

    // Undo: y ^= y >> L;
    y ^= (y & !(0xFFFFFFFF >> L)) >> L;

    // Undo: y ^= (y << T) & C;
    y ^= ((y & !((0xFFFFFFFF << T) & C)) << T) & C;

    // Undo: y ^= (y << S) & B;
    {
        let mask = !(0xFFFFFFFF << S);

        let a = y & mask;
        let b = (y ^ ((a << S) & B)) & (mask << S) | a;
        let c = (y ^ ((b << S) & B)) & (mask << (2 * S)) | b;
        let d = (y ^ ((c << S) & B)) & (mask << (3 * S)) | c;
        y ^= (d << S) & B;
    }

    // Undo: y ^= (y >> U) & D;
    {
        let a = y & !(0xFFFFFFFF >> U);
        let b = (y ^ (a >> U)) & !(0xFFFFFFFF >> (2 * U));
        let c = y ^ (b >> U);

        y = c;
    }

    y
}
