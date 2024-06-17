
use crate::merkle_tree::{MerkleTree, compare_merkle_trees};

pub fn image_verification(fake_merkle_tree: MerkleTree, original_merkle_tree: MerkleTree) -> Vec<u32>{
    let ri = compare_merkle_trees(&original_merkle_tree, &fake_merkle_tree);
    println!("Tampered result array: {:?} {:?}", ri,ri.len());
    ri
}
