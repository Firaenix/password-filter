use std::{fs::File, io::{self, BufRead}};

use xorf::{Filter, BinaryFuse8};


fn main() {
    let read_bytes = std::fs::read("./xorfilter.bin").unwrap();

    let reconst_filter: BinaryFuse8 = bincode::deserialize(&read_bytes).unwrap();

    let mut sha1 = sha1_smol::Sha1::new();
    sha1.update(b"21e8");
    let sha_hex = sha1.digest().to_string();

    let bytes = hex::decode(&sha_hex).unwrap();

    let prefix: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], 
    bytes[4], bytes[5], bytes[6], bytes[7]];
    let hexval = u64::from_be_bytes(prefix);

    let contains = reconst_filter.contains(&hexval);

    println!("FILTER CONTAINS {} {}", hexval, contains)
}