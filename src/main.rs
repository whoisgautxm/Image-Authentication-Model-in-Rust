mod ImageToMSB;
mod ImageToChunks;
mod BlockEncryption;
// mod ipfs;

use ImageToMSB::extract_msb;
use ImageToChunks::slice_image_into_blocks;
use BlockEncryption::{encrypt_block, save_to_file};
// use ipfs::upload_to_ipfs;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rand::rngs::OsRng;
use std::path::Path;

fn generate_rsa_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

#[tokio::main]
async fn main() {
    let (private_key, public_key) = generate_rsa_keys();

    extract_msb();

    let img = image::open("/Users/shivanshgupta/Documents/Coding Projects/Image-Authentication-Model-in-Rust/msb_image1.jpg").unwrap().to_rgba8();
    let block_size = 128;
    let blocks = slice_image_into_blocks(&img, block_size);

    for (i, block) in blocks.iter().enumerate() {
        let encrypted_block = encrypt_block(block, &public_key);
        let file_name = format!("block_{}.enc", i + 1);
        save_to_file(&encrypted_block, Path::new(&file_name));
        println!("Saved {}", file_name);
    }

}
