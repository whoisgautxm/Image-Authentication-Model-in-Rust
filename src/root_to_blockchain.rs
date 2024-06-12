use rs_merkle::{MerkleTree, algorithms::Sha256, Hasher};

pub fn calculate_root(leaves: Vec<String>) {
    // Specify that we are collecting into a Vec<[u8; 32]>
    let leaves_as_bytes: Vec<[u8; 32]> = leaves.iter().map(|x| Sha256::hash(x.as_bytes())).collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves_as_bytes);

    // Handle the error case of the root being None
    match merkle_tree.root_hex() {
        Some(root) => println!("This is the root node: {:?}", root),
        None => eprintln!("Couldn't get the merkle root"),
    }
}
