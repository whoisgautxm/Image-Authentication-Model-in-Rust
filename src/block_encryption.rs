use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes128;
use ctr::Ctr128BE;
use image::{ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path;

type Aes128Ctr = Ctr128BE<Aes128>;

// Function to derive key and nonce from image data
pub fn derive_key_nonce_from_image(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.update(image.as_raw());
    let result = hasher.finalize();
    let key = result[0..16].to_vec();
    let nonce = result[16..32].to_vec(); // Using 12 bytes for nonce as recommended for AES-GCM
    println!("Key: {:x?}, Nonce: {:x?}", key, nonce); // Debugging info
    (key, nonce)
}

pub fn encrypt_block(block: &ImageBuffer<Rgba<u8>, Vec<u8>>, key: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut cipher = Aes128Ctr::new(key.into(), nonce.into());
    let mut block_data = block.as_raw().clone();
    cipher.apply_keystream(&mut block_data);
    println!("Encrypted Block Data: {:x?}", &block_data[..16]); // Debugging info
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

pub fn decrypt_block(data: &[u8], key: &[u8], nonce: &[u8], block_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut cipher = Aes128Ctr::new(key.into(), nonce.into());
    let mut decrypted_data = data.to_vec();
    cipher.apply_keystream(&mut decrypted_data);
    println!("Decrypted Block Data: {:x?}", &decrypted_data[..16]); // Debugging info

    let mut block = ImageBuffer::new(block_size, block_size);
    for (i, pixel) in decrypted_data.chunks(4).enumerate() {
        let x = (i as u32) % block_size;
        let y = (i as u32) / block_size;
        block.put_pixel(x, y, Rgba([pixel[0], pixel[1], pixel[2], pixel[3]]));
    }
    block
}
