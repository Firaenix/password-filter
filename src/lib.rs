use wasm_bindgen::prelude::*;
use xorf::{BinaryFuse8, Filter};
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn is_known_compromised_password(password: &str) -> bool {
    console_error_panic_hook::set_once();

    let read_bytes = include_bytes!("../xorfilter.bin");

    let reconst_filter: BinaryFuse8 = bincode::deserialize(read_bytes).unwrap();

    let mut sha1 = sha1_smol::Sha1::new();
    sha1.update(password.as_bytes());
    let sha_hex = sha1.digest().to_string();

    let bytes = hex::decode(&sha_hex).unwrap();

    let prefix: [u8; 8] = [bytes[0], bytes[1], bytes[2], bytes[3], 
    bytes[4], bytes[5], bytes[6], bytes[7]];
    let hexval = u64::from_be_bytes(prefix);

    reconst_filter.contains(&hexval)
}