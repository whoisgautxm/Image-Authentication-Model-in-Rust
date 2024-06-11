mod ImageToMSB;
mod ImageToChunks;
mod BlockEncryption;

use ImageToMSB::extract_msb;
use ImageToChunks::slice_image_into_blocks;
use BlockEncryption::{encrypt_block, save_to_file};
use std::path::Path;

fn main() {
    // Extract MSB from original image and create image from MSBs
    let msb_img = ImageToMSB::extract_msb("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/rdr.jpg");

    // Break the new image into blocks
    let block_size = 128;
    let blocks = ImageToChunks::slice_image_into_blocks(&msb_img, block_size);

    // Define a key and nonce for AES encryption
    let key = b"an_example_key_1";
    let nonce = b"an_example_nonce";

    // Encrypt each block and save to file
    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = BlockEncryption::encrypt_block(block, key, nonce);
        let file_name = format!("block_{}.enc", i + 1);
        BlockEncryption::save_to_file(&encrypted_block, Path::new(&file_name));
        println!("Saved {}", file_name);
    }
}
