// src/image_verification.rs

use crate::merkle_tree::{MerkleTree, ri_array};

pub fn image_verification(fake_merkle_tree: MerkleTree, original_merkle_tree: MerkleTree) {
    let ri = ri_array(&original_merkle_tree, &fake_merkle_tree);
    println!("Tampered result array: {:?}", ri);
}
