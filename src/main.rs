use rand::Rng;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let MB = 20;
    let size = (MB * 2_u32.pow(20));
    let random_bytes: Vec<u8> = (0..size).map(|_| { rand::random::<u8>() }).collect();
    let mut buffer = File::create("foo.txt").expect("failed to create file");

    buffer.write(&random_bytes).expect("failed to wrtie to file");
}
