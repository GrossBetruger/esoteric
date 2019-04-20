use rand::Rng;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let MB = 25;
    let size = (MB * 2_u32.pow(20));
    let random_bytes: Vec<u8> = (0..size).map(|_| { rand::random::<u8>() }).collect();

    let mut hasher = Sha256::new();
    hasher.input(&random_bytes);
    let hex_digest = hasher.result_str();
    let mut buffer = File::create(&hex_digest).expect("failed to create file");
    println!("{}", hex_digest);
    buffer.write(&random_bytes).expect("failed to wrtie to file");
}
