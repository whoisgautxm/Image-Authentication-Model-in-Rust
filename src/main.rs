mod image_to_msb;
mod image_to_chunks;
mod block_encryption;
mod ipfs_upload;
mod root_to_blockchain;

use image_to_msb::extract_msb;
use image_to_chunks::slice_image_into_blocks;
use block_encryption::{encrypt_and_save_blocks, generate_key_nonce};
use root_to_blockchain::calculate_root;
use ipfs_upload::upload_to_ipfs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Extract MSB from original image and create image from MSBs
    let msb_img = extract_msb("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/rdr.jpg");

    // Break the new image into blocks
    let block_size = 32;
    let blocks = slice_image_into_blocks(&msb_img, block_size);

    // Define a key and nonce for AES encryption
    let (key, nonce) = generate_key_nonce();

    // Encrypt each block and save to file
    encrypt_and_save_blocks(&blocks, &key, &nonce);

    // vector for storing hashes of the blocks as leaves
    let mut leaves:Vec<String> =  Vec::new();

    // Upload encrypted blocks to IPFS and get their hashes
    for i in 0..blocks.len() {
        let file_name = format!("block_{}.enc", i + 1);
        let file_path = Path::new(&file_name);
    

        match upload_to_ipfs(file_path).await {
            Ok(hash) => {println!("Uploaded to IPFS with hash and block_no{}: {}", i+1, hash);
            leaves.push(hash)
        }
            Err(e) => eprintln!("Error uploading to IPFS: {}", e),
        }
    }
    // this function will calculate root of the merkle tree from the leaves
    calculate_root(leaves);

}