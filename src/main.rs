mod image_to_msb;
mod image_to_chunks;
mod block_encryption;
mod ipfs_upload;
mod merkle_tree;
mod blockchain;
mod image_verification;

use image_to_msb::{extract_msb, convert_msb_to_normal};
use image_to_chunks::slice_image_into_blocks;
use block_encryption::{encrypt_and_save_blocks, decrypt_block};
use merkle_tree::{insert_root,  build_tree, MerkleTree};
use ipfs_upload::{upload_to_ipfs, download_file_from_ipfs};
use blockchain::{Blockchain, return_transaction};
use image_verification::image_verification;
use std::path::Path;
use image::{GenericImageView, ImageBuffer, Rgba};

#[tokio::main]
async fn main() {
    // Define the images to process and their corresponding prefixes
    let original_image_path = "/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/image2.jpg";
    let original_prefix = "original";
    let deprecated_image_path = "/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/image3.jpg";
    let deprecated_prefix = "fake";
    let block_size = 128;

    // Process both images
    let leaves_original = process_image(original_image_path, block_size, original_prefix).await;
    let leaves_fake = process_image(deprecated_image_path, block_size, deprecated_prefix).await;

    // Initialize a blockchain
    let mut blockchain = Blockchain::new();

    // Insert leaves_original in the Transaction of the blockchain
    insert_root(leaves_original.clone(), &mut blockchain);

    // Calculate fake merkle tree and return it
    let fake_merkle_tree = build_tree(leaves_fake);

    // Get the transaction of the block by calculating the hash of the header
    let last_block_hash = blockchain::calculate_hash(&blockchain.chain.last().unwrap().header);

    // Return leaves of the original image
    let original_transactions = return_transaction(&blockchain, &last_block_hash);

    // Merkle tree from original leaves
    let original_merkle_tree = build_tree(original_transactions);

    // Perform image verification and get the `ri` array
    let ri = image_verification(fake_merkle_tree, original_merkle_tree);

    // Restore the tampered blocks
    let restored_image = restore_tampered_blocks(original_image_path, &leaves_original, &ri, block_size).await;

    // Save the restored image
    restored_image.save("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/image4.png").expect("Failed to save restored image");
}

// Function to process an image: extract MSB, slice into blocks, encrypt, upload to IPFS, and collect hashes
async fn process_image(image_path: &str, block_size: u32, prefix: &str) -> Vec<String> {
    // Extract MSB from image and create image from MSBs
    let msb_img = extract_msb(image_path);

    // Break the image into blocks
    let blocks = slice_image_into_blocks(&msb_img, block_size);

    // Encrypt each block and save to file with the given prefix
    encrypt_and_save_blocks(&blocks, prefix);

    // Vector for storing hashes of the blocks as leaves
    let mut leaves = Vec::new();

    // Upload encrypted blocks to IPFS and get their hashes
    for i in 0..blocks.len() {
        let file_name = format!("{}_block_{}.enc", prefix, i + 1);
        let file_path = Path::new(&file_name);
        match upload_to_ipfs(file_path).await {
            Ok(hash) => {
                println!("Uploaded to IPFS with Block_no and Hash {}: {}", i + 1, hash);
                leaves.push(hash);
            }
            Err(e) => eprintln!("Error uploading to IPFS: {}", e),
        }
    }

    leaves
}

// Function to restore tampered blocks
async fn restore_tampered_blocks(original_image_path: &str, leaves_original: &[String], ri: &Vec<u32>, block_size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Load the original image
    let original_image = image::open(original_image_path).expect("Failed to open original image");

    // Convert DynamicImage to ImageBuffer<Rgba<u8>, Vec<u8>>
    let original_image_buffer = original_image.to_rgba8();
    let (width, height) = original_image_buffer.dimensions();
    let mut restored_image = ImageBuffer::new(width, height);

    // Define transparent red color
    let transparent_red = Rgba([255, 0, 0, 128]);

    // Iterate over the `ri` array
    for (i, &r) in ri.iter().enumerate() {
        // Calculate the position of the block in the image
        let x = (i as u32 % (width / block_size)) * block_size;
        let y = (i as u32 / (width / block_size)) * block_size;

        if r == 1 {
            let tx_hash = &leaves_original[i];

            // Download and decrypt the file from IPFS
            let encrypted_block = download_file_from_ipfs(tx_hash).await.expect("Failed to download from IPFS");
            
            let decrypted_block = decrypt_block(&encrypted_block, block_size);
            
            // Save decrypted block for debugging
            let file_name = format!("Decrypted_block_MSB{}.png", i + 1);
            decrypted_block.save(&file_name).expect("Failed to save decrypted block");

            // restore original format image from msb_image
            let  original_decrypted_block = convert_msb_to_normal(&decrypted_block);

            let file_name = format!("Decrypted_block_{}.png", i + 1);
            original_decrypted_block.save(&file_name).expect("Failed to save decrypted block");

            let x = (i as u32 % (width / block_size)) * block_size;
            let y = (i as u32 / (width / block_size)) * block_size;

            // Fill the block with transparent red
            for by in 0..block_size {
                for bx in 0..block_size {
                    if x + bx < width && y + by < height {
                        let pixel = original_decrypted_block.get_pixel(bx, by);
                        restored_image.put_pixel(x + bx, y + by, *pixel);
                    }
                }
            }
        } else {
            // Copy the block from the original image to the restored image
            for by in 0..block_size {
                for bx in 0..block_size {
                    if x + bx < width && y + by < height {
                        let pixel = original_image_buffer.get_pixel(x + bx, y + by);
                        restored_image.put_pixel(x + bx, y + by, *pixel);
                    }
                }
            }
        }
    }

    restored_image
}

// fn main(){
//     let msb_img = extract_msb("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/image2.jpg");
//     let original_image= convert_msb_to_normal(&msb_img);
// }