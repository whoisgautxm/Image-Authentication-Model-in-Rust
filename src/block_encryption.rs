use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};
use image::{ImageBuffer, Rgba};
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Write;
use std::path::Path;

type Aes128Ctr = ctr::Ctr128BE<Aes128>;

// Function to derive key and nonce from image data
pub fn derive_key_nonce_from_image(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.update(image.as_raw());
    let result = hasher.finalize();
    let key = result[0..16].to_vec();
    let nonce = result[16..32].to_vec();
    (key, nonce)
}
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

pub fn encrypt_and_save_blocks_with_derived_keys(blocks: &[ImageBuffer<Rgba<u8>, Vec<u8>>], prefix: &str) {
    for (i, block) in blocks.iter().enumerate() {
        let (key, nonce) = derive_key_nonce_from_image(block);
        let encrypted_block = encrypt_block(block, &key, &nonce);
        let file_name = format!("{}_block_{}.enc", prefix, i + 1);
        save_to_file(&encrypted_block, Path::new(&file_name));
        println!("Saved {}", file_name);
    }
}
