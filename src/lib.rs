use wasm_bindgen::prelude::*;
use xorf::{BinaryFuse8, Filter};
extern crate console_error_panic_hook;

#[wasm_bindgen(js_name = isCompromisedPassword)]
pub fn is_compromised_password(password: &str) -> bool {
    console_error_panic_hook::set_once();

    let read_bytes = include_bytes!("../xorfilter.bin");

    let reconst_filter: BinaryFuse8 = bincode::deserialize(read_bytes).unwrap();

    let mut sha1 = sha1_smol::Sha1::new();
    sha1.update(password.as_bytes());
    let sha_hex = sha1.digest().to_string();

    let hexval = match u64::from_str_radix(&sha_hex[0..16], 16) {
        Ok(v) => v,
        Err(_) => return false
    };

    reconst_filter.contains(&hexval)
}