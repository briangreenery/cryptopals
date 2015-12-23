#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn main() {
    for i in 0..0xFFFFFFFFu64 {
        let start = i as u32;
        let temper = pals::mt19937::temper(start);
        let untemper = pals::mt19937::untemper(temper);
        
        if untemper != start {
            println!("WHOOPS! mismatch at {}", i);
            break;
        }
        
        if i % (32 * 1024 * 1024) == 0 {
            println!("checked up to {}", i);
        }
    }
}
