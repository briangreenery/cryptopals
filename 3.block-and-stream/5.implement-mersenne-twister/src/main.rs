#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

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

struct MT19937 {
    state: [u32; N],
    index: usize,
}

impl MT19937 {
    fn new(seed: u32) -> MT19937 {
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

    fn gen(&mut self) -> u32 {
        if self.index >= N {
            self.twist();
            self.index = 0;
        }

        let mut y = self.state[self.index];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index += 1;
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


fn main() {
    let mut rand = MT19937::new(587);

    for _i in 0..10 {
        println!("{}", rand.gen());
    }
}
