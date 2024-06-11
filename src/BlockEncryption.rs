use rsa::{RsaPublicKey, PaddingScheme, PublicKey};
use rand::rngs::OsRng;
use image::ImageBuffer;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use base64::{encode, decode};
use image::Rgba;

pub fn encrypt_block(block: &ImageBuffer<Rgba<u8>, Vec<u8>>, public_key: &RsaPublicKey) -> Vec<u8> {
    let mut rng = OsRng;
    let block_data = block.as_raw();
    let ciphertext = public_key.encrypt(&mut rng, PaddingScheme::PKCS1v15Encrypt, block_data).expect("Failed to encrypt");
    ciphertext
}

pub fn save_to_file(data: &[u8], path: &Path) {
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(data).expect("Failed to write data to file");
}
