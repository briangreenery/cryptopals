#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;

fn test() {
    for i in 0..0xFFFFFFFFu64 {
        let start = i as u32;
        let temper = pals::mt19937::temper(start);
        let untemper = pals::mt19937::untemper(temper);

        assert_eq!(untemper, start);

        if i % (256 * 1024 * 1024) == 0 {
            println!("verified up to {}", i);
        }
    }

    println!("");
}

fn main() {
    test();

    let mut original = pals::MT19937::new(1234);

    let mut output = [0; 624];
    for i in 0..output.len() {
        output[i] = original.gen();
    }

    let mut copy = pals::MT19937::clone_from(&output);

    for _i in 0..10 {
        println!("original: {}", original.gen());
        println!("copy:     {}", copy.gen());
        println!("");
    }
}
