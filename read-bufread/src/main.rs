use std::env::args;
use std::fs::{metadata, File};
use std::io::{BufReader, Read};

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");
    let file = File::open(path).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    // BufReader is the most common solution suggested for efficent reads of
    // large files where performance matters. Weirdly a smaller block size
    // than read-chunks is more efficient.
    const BLOCK_SIZE: usize = 2_097_152; //2M
    let mut contents = vec![0_u8; BLOCK_SIZE];
    let mut read_length: usize = 0;
    for _ in 0..=(length / BLOCK_SIZE) {
        // We don't want to read lines for this experiment to keep it
        // consistent with the others even though that's the most common use of
        // BufReader.
        read_length += reader.read(&mut contents).expect("Couldn't read file");
    }

    println!("{} {}", read_length, length);
}
