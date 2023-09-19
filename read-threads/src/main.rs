use std::env::args;
use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom};
use std::thread;

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let length: usize = metadata(path)
        .expect("Unable to query file details")
        .len()
        .try_into()
        .expect("Couldn't convert len from u64 to usize");

    const BLOCK_SIZE: usize = 16_777_216; //16M
    const THREADS: usize = 10;
    // How much each thread should read
    let division: usize = ((length / THREADS) as f64).ceil() as usize;

    // Use scoped threads to keep things simpler
    thread::scope(|scope| {
        for i in 0..THREADS {
            scope.spawn(move || {
                // Open a file handle per thread
                let mut thread_file = File::open(&path).expect("Unable to open file");
                let mut contents = vec![0_u8; BLOCK_SIZE];
                // Can't be zero since that's the EOF condition from read()
                let mut read_length: usize = 1;
                let mut read_total: usize = 0;
                let offset: u64 = (i * division) as u64;
                thread_file
                    .seek(SeekFrom::Start(offset))
                    .expect("Couldn't seek to position in file");

                while (read_total < division) && (read_length != 0) {
                    // Handle the case when the bytes remaining to be read are
                    // less than the block size
                    if read_total + BLOCK_SIZE > division {
                        contents.truncate(division - read_total);
                    }
                    read_length = thread_file.read(&mut contents).expect("Couldn't read file");
                    read_total += read_length;
                }

                println!("{} {}", read_total, length);
            });
        }
    });
}
