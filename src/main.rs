mod image_to_msb;
mod image_to_chunks;
mod block_encryption;

use image_to_msb::extract_msb;
use image_to_chunks::slice_image_into_blocks;
use block_encryption::encrypt_and_save_blocks;

fn main() {
    // Extract MSB from original image and create image from MSBs
    let msb_img = extract_msb("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/rdr.jpg");

    // Break the new image into blocks
    let block_size = 32;
    let blocks = slice_image_into_blocks(&msb_img, block_size);

    // Define a key and nonce for AES encryption
    let key = b"an_example_key_1";
    let nonce = b"an_example_nonce";

    // Encrypt each block and save to file
    encrypt_and_save_blocks(&blocks, key, nonce);
}
