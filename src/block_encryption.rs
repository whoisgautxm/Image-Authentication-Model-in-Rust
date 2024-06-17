use aes::cipher::{KeyIvInit, StreamCipher};
use aes::Aes128;
use ctr::Ctr128BE;
use image::{ImageBuffer, Rgba};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;
use std::path::Path;

type Aes128Ctr = Ctr128BE<Aes128>;

// Simple XOR cipher for encryption and decryption
fn xor_cipher(data: &mut [u8], key: u8) {
    for byte in data.iter_mut() {
        *byte ^= key;
    }
}

pub fn encrypt_block(block: &ImageBuffer<Rgba<u8>, Vec<u8>>, key: u8) -> Vec<u8> {
    let mut block_data = block.as_raw().clone();
    xor_cipher(&mut block_data, key);
    block_data
}

pub fn save_to_file(data: &[u8], path: &Path) {
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write data to file");
}

pub fn encrypt_and_save_blocks(blocks: &[ImageBuffer<Rgba<u8>, Vec<u8>>], prefix: &str) {
    let key = 0xAA; // Example key for XOR encryption
    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = encrypt_block(block, key);
        let file_name = format!("{}_block_{}.enc", prefix, i + 1);
        save_to_file(&encrypted_block, Path::new(&file_name));
        
        // Calculate and print hash for debugging
        let mut hasher = Sha256::new();
        hasher.update(&encrypted_block);
        let hash = hasher.finalize();
        println!("Block {}: Encrypted hash: {}", i + 1, hex::encode(hash));

        println!("Saved {}", file_name);
    }
}

pub fn decrypt_block(data: &[u8], block_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let key = 0xAA; // Example key for XOR decryption

    // Perform XOR decryption
    let mut decrypted_data = data.to_vec();
    xor_cipher(&mut decrypted_data, key);

    // Reconstruct the image buffer from decrypted data
    let mut block = ImageBuffer::new(block_size, block_size);
    for (i, pixel) in decrypted_data.chunks(4).enumerate() {
        let x = (i as u32) % block_size;
        let y = (i as u32) / block_size;
        
        // Ensure bounds checking for pixel data
        if let [r, g, b, a] = pixel {
            block.put_pixel(x, y, Rgba([*r, *g, *b, *a]));
        } else {
            // Handle unexpected chunk size
            eprintln!("Unexpected chunk size during pixel reconstruction.");
        }
    }
    block
}

