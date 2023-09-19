use std::env::args;
use std::fs::{metadata, read_to_string};

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");

    // The common advice given when you google "reading a file in rust"
    // From https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let contents = read_to_string(path)
        .expect("Should have been able to read the file");

    println!("{} {}", contents.len(), length);
}
