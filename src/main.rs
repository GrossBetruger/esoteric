extern crate xorshift;
extern crate time;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::fs::File;
use xorshift::{Rng, SeedableRng, Xorshift128};
use time::precise_time_ns;
use std::io::prelude::*;


fn main() {
    let now = precise_time_ns();
    // Manually seed a Xorshift128+ PRNG
    let states = [now, now];
    let mut rng: Xorshift128 = SeedableRng::from_seed(&states[..]);

    let megabytes = 25;
    let size = megabytes * 2_u32.pow(20);
    let random_bytes: Vec<u8> = (0..size).map(|_| { rng.next_u64() as u8 }).collect();


    let mut hasher = Sha256::new();
    hasher.input(&random_bytes);
    let hex_digest = hasher.result_str();
    let mut buffer = File::create(&hex_digest).expect("failed to create file");
    println!("{}", hex_digest);
    buffer.write(&random_bytes).expect("failed to wrtie to file");
}
