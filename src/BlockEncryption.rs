use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};
use image::{ImageBuffer, Rgba};
use std::fs::File;
use std::io::Write;
use std::path::Path;

type Aes128Ctr = ctr::Ctr128BE<Aes128>;

pub fn encrypt_block(block: &ImageBuffer<Rgba<u8>, Vec<u8>>, key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut cipher = Aes128Ctr::new(key.into(), nonce.into());
    let mut block_data = block.as_raw().clone();
    cipher.apply_keystream(&mut block_data);
    block_data
}

pub fn save_to_file(data: &[u8], path: &Path) {
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write data to file");
}
