#![allow(dead_code)]
mod pals;

extern crate crypto;
extern crate rand;
extern crate time;

use rand::Rng;

fn wait_random_time() {
    let seconds = rand::thread_rng().gen_range(40, 1000);
    let duration = std::time::Duration::from_secs(seconds);
    std::thread::sleep(duration);
}

fn now_unix_time() -> u32 {
    time::now_utc().to_timespec().sec as u32
}

fn main() {
    loop {
        wait_random_time();
        let mut twister = pals::MT19937::new(now_unix_time());
        wait_random_time();
        println!("{}", twister.gen());
    }
}
