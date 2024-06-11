// Import the necessary modules and types
use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};
use image::{ImageBuffer, Rgba};
use rand::RngCore; // Import the RngCore trait for random number generation
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Define a type alias for AES-128 in CTR mode with a 128-bit block size
type Aes128Ctr = ctr::Ctr128BE<Aes128>;


// Function to generate a secure random key and nonce
fn generate_key_nonce() -> (Vec<u8>, Vec<u8>) {
    let mut key = vec![0u8; 16]; // 16 bytes for AES-128 key
    let mut nonce = vec![0u8; 16]; // 16 bytes for AES-128 nonce

    // Generate secure random key and nonce
    rand::thread_rng().fill_bytes(&mut key);
    rand::thread_rng().fill_bytes(&mut nonce);

    (key, nonce)
}


// Function to encrypt a single image block
pub fn encrypt_block(block: &ImageBuffer<Rgba<u8>, Vec<u8>>, key: &[u8], nonce: &[u8]) -> Vec<u8> {
    // Initialize the AES-128-CTR cipher with the given key and nonce
    let mut cipher = Aes128Ctr::new(key.into(), nonce.into());

    // Clone the raw byte data of the image block
    let mut block_data = block.as_raw().clone();

    // Apply the encryption keystream to the block data
    cipher.apply_keystream(&mut block_data);

    // Return the encrypted block data
    block_data
}

// Function to save data to a file at the specified path
pub fn save_to_file(data: &[u8], path: &Path) {
    // Create a new file at the specified path, expecting the operation to succeed
    let mut file = File::create(path).expect("Failed to create file");

    // Write all the data to the file, expecting the operation to succeed
    file.write_all(data).expect("Failed to write data to file");
}

// Function to encrypt and save multiple image blocks
pub fn encrypt_and_save_blocks(blocks: &[ImageBuffer<Rgba<u8>, Vec<u8>>], key: &[u8], nonce: &[u8]) {
    // Iterate over the image blocks with their indices
    for (i, block) in blocks.iter().enumerate() {
        // Encrypt the current block
        let encrypted_block = encrypt_block(block, key, nonce);

        // Create a file name based on the block index
        let file_name = format!("block_{}.enc", i + 1);

        // Save the encrypted block data to the file
        save_to_file(&encrypted_block, Path::new(&file_name));

        // Print a message indicating the file was saved
        println!("Saved {}", file_name);
    }
}
