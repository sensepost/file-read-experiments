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

    // Raw io byte read of whole file into a single buffer
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = vec![0_u8; length];
    let len: usize = file.read_to_end(&mut contents).expect("Couldn't read file");

    println!("{} {}", len, length);
}
