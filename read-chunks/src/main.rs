use std::env::args;
use std::fs::{metadata, File};
use std::io::Read;

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");
    let mut file = File::open(path).expect("Unable to open file");

    // Block size optimal for my 2021 MBP M1
    const BLOCK_SIZE: usize = 8_388_608; //8M
    let mut contents = vec![0_u8; BLOCK_SIZE];
    let mut read_length: usize = 0;
    for _ in 0..=(length / BLOCK_SIZE) {
        read_length += file.read(&mut contents).expect("Couldn't read file");
    }

    println!("{} {}", read_length, length);
}
