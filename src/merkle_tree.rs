// src/merkle_tree.rs
use sha2::{Digest, Sha256};
use std::{fmt::{self, Debug, Formatter}};
use hex;
use crate::Blockchain;

#[derive(Clone)]
struct Node {
    hash: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    num_leaves: usize, // Number of leaves under this node
}

impl Node {
    fn new(hash: Vec<u8>, num_leaves: usize) -> Self {
        Node {
            hash,
            left: None,
            right: None,
            num_leaves,
        }
    }

    fn print(&self, depth: usize) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", hex::encode(&self.hash));

        if let Some(ref left) = self.left {
            left.print(depth + 1);
        }
        if let Some(ref right) = self.right {
            right.print(depth + 1);
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.hash))
    }
}

pub struct MerkleTree {
    pub root: Option<Node>,
}

impl MerkleTree {
    pub fn new(data: Vec<&str>) -> Self {
        let leaves: Vec<Node> = data
            .into_iter()
            .map(|datum| {
                let mut hasher = Sha256::new();
                hasher.update(datum);
                let hash = hasher.finalize().to_vec();
                Node::new(hash, 1) // Each leaf node has 1 leaf
            })
            .collect();

        let root = Self::build_tree(leaves);

        MerkleTree { root }
    }

    fn build_tree(mut nodes: Vec<Node>) -> Option<Node> {
        while nodes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in nodes.chunks(2) {
                let left = chunk[0].clone();
                let right = if chunk.len() == 2 { chunk[1].clone() } else { chunk[0].clone() };

                let mut hasher = Sha256::new();
                hasher.update(&left.hash);
                hasher.update(&right.hash);
                let parent_hash = hasher.finalize().to_vec();

                let len = next_level.len() + 1 ;

                let num_leaves = left.num_leaves+right.num_leaves; // Sum of leaves under the left and right nodes

                let mut parent_node = Node::new(parent_hash, num_leaves);
                parent_node.left = Some(Box::new(left));
                parent_node.right = Some(Box::new(right));

                next_level.push(parent_node);
            }

            nodes = next_level;
        }

        nodes.into_iter().next()
    }

    pub fn print_tree(&self) {
        if let Some(ref root) = self.root {
            root.print(0);
        }
    }

    pub fn traverse(&self, callback: &mut dyn FnMut(&Node)) {
        if let Some(ref root) = self.root {
            Self::traverse_node(root, callback);
        }
    }

    fn traverse_node(node: &Node, callback: &mut dyn FnMut(&Node)) {
        callback(node);

        if let Some(ref left) = node.left {
            Self::traverse_node(left, callback);
        }

        if let Some(ref right) = node.right {
            Self::traverse_node(right, callback);
        }
    }

    pub fn root_hex(&self) -> Option<String> {
        self.root.as_ref().map(|node| hex::encode(&node.hash))
    }

    pub fn size(&self) -> usize {
        let mut count = 0;
        self.traverse(&mut |_| count += 1);
        count
    }
}

fn compare_nodes(node1: &Option<Node>, node2: &Option<Node>, ri: &mut Vec<u32>) {
    match (node1, node2) {
        (Some(n1), Some(n2)) => {
            if n1.hash != n2.hash {
                if n1.left.is_none() && n1.right.is_none() && n2.left.is_none() && n2.right.is_none() {
                    // Both nodes are leaves
                    ri.push(1);
                } else {
                    // Recursively compare children
                    compare_nodes(&n1.left.as_deref().cloned(), &n2.left.as_deref().cloned(), ri);
                    compare_nodes(&n1.right.as_deref().cloned(), &n2.right.as_deref().cloned(), ri);
                }
            } else if n1.left.is_none() && n1.right.is_none() && n2.left.is_none() && n2.right.is_none() {
                ri.extend(vec![0; n1.num_leaves]);
            }
            else if n1.left.is_some() && n1.right.is_some() && n2.left.is_some() && n2.right.is_some(){
                ri.extend(vec![0; n1.num_leaves]);
            }
        }
        _ => {}
    }
}

pub fn ri_array(original_merkle: &MerkleTree, fake_merkle: &MerkleTree) -> Vec<u32> {
    let mut ri = Vec::new();
    
    // Check if the root hashes match
    let roots_match = original_merkle.root_hex() == fake_merkle.root_hex();
    
    // If roots match, populate ri with 0s only for leaf nodes
    if roots_match {
        original_merkle.traverse(&mut |node| {
            if node.left.is_none() && node.right.is_none() {
                ri.push(0);
            }
        });
    } else {
        // If roots don't match, perform comparison as before
        compare_nodes(&original_merkle.root, &fake_merkle.root, &mut ri);
    }

    ri
}

pub fn insert_root(leaves_original: Vec<String>, blockchain: &mut Blockchain) {
    let leaves_as_str_original: Vec<&str> = leaves_original.iter().map(|s| s.as_str()).collect();
    let merkle_tree = MerkleTree::new(leaves_as_str_original.clone());

    match merkle_tree.root_hex() {
        Some(root) => {
            println!("This is the root node: {:?}", root);
            merkle_tree.print_tree();
            merkle_tree.traverse(&mut |node| println!("{}", hex::encode(&node.hash)));

            blockchain.add_block(root, leaves_original)
        }
        None => eprintln!("Couldn't get the merkle root"),
    }
}

pub fn build_original_tree(leaves_original: Vec<String>) -> MerkleTree {
    let leaves_as_str_original: Vec<&str> = leaves_original.iter().map(|s| s.as_str()).collect();
    let merkle_tree = MerkleTree::new(leaves_as_str_original.clone());
    merkle_tree
}

pub fn build_fake_tree(leaves_fake: Vec<String>) -> MerkleTree {
    let leaves_as_str_fake: Vec<&str> = leaves_fake.iter().map(|s| s.as_str()).collect();
    let fake_merkle_tree = MerkleTree::new(leaves_as_str_fake.clone());
    fake_merkle_tree
}
