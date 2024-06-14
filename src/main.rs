mod image_to_msb;
mod image_to_chunks;
mod block_encryption;
mod ipfs_upload;
mod root_to_blockchain;

use image_to_msb::extract_msb;
use image_to_chunks::slice_image_into_blocks;
use block_encryption::{encrypt_and_save_blocks_with_derived_keys, derive_key_nonce_from_image};
use root_to_blockchain::calculate_root;
use ipfs_upload::upload_to_ipfs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Define the images to process and their corresponding prefixes
    let original_image_path = "../image.png";
    let original_prefix = "original";
    let deprecated_image_path = "../image1.png";
    let deprecated_prefix = "fake";
    let block_size = 32;

    // Process both images
    let leaves_original = process_image(original_image_path, block_size, original_prefix).await;
    let leaves_fake = process_image(deprecated_image_path, block_size, deprecated_prefix).await;

    // Calculate Merkle roots
    calculate_root(leaves_original,leaves_fake);


}

// Function to process an image: extract MSB, slice into blocks, encrypt, upload to IPFS, and collect hashes
async fn process_image(image_path: &str, block_size: u32, prefix: &str) -> Vec<String> {
    // Extract MSB from image and create image from MSBs
    let msb_img = extract_msb(image_path);

    // Break the image into blocks
    let blocks = slice_image_into_blocks(&msb_img, block_size);

    // Define a key and nonce for AES encryption
    let (key, nonce) = derive_key_nonce_from_image(&msb_img);

    // Encrypt each block and save to file with the given prefix
    encrypt_and_save_blocks_with_derived_keys(&blocks, prefix);

    // Vector for storing hashes of the blocks as leaves
    let mut leaves = Vec::new();

    // Upload encrypted blocks to IPFS and get their hashes
    for i in 0..blocks.len() {
        let file_name = format!("{}_block_{}.enc", prefix, i + 1);
        let file_path = Path::new(&file_name);
        match upload_to_ipfs(file_path).await {
            Ok(hash) => {
                println!("Uploaded to IPFS with hash and block_no {}: {}", i + 1, hash);
                leaves.push(hash);
            }
            Err(e) => eprintln!("Error uploading to IPFS: {}", e),
        }
    }

    leaves
}