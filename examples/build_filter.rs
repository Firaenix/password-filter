use std::{io::{self, BufRead, Cursor}, fs::File, collections::HashMap, time::Instant};
use xorf::{Filter, BinaryFuse8};

const SAMPLE_SIZE: usize = 1_000_000;

fn main() {
    // Open a file
    let file = File::open("./dataset/pwned-passwords-sha1-ordered-by-count.txt").unwrap();
    let mut file_lines = io::BufReader::with_capacity(SAMPLE_SIZE, file).lines().enumerate();

    let mut keys = HashMap::<u64, ()>::with_capacity(SAMPLE_SIZE);
    // let mut keys = [0; SAMPLE_SIZE];
    
    let start = Instant::now();
    while let Some((idx , Ok(line))) = file_lines.next() {
        if idx % 10_000 == 0{
            println!("Read {} keys", idx);
        }

        if idx == SAMPLE_SIZE {
            println!("Read {} keys, continuing", SAMPLE_SIZE);
            break;
        }

        let hexval =  u64::from_str_radix(&line[0..16], 16).unwrap();
        keys.insert(hexval, ());
        // keys[idx] = hexval;
    }    

    println!("Took {}ms", start.elapsed().as_millis());

    let keys = Vec::from_iter(keys.into_keys());
    let filter = BinaryFuse8::try_from(&keys.to_vec()).unwrap();

    let bytes = bincode::serialize(&filter).unwrap();

    std::fs::write("./xorfilter.bin", bytes).unwrap();
}