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

//-------------------------------------------------------------------- MERKLE TREE COMPARISON: START --------------------------------------------------------------------

pub fn compare_merkle_trees(tree1: &MerkleTree, tree2: &MerkleTree) -> Vec<u32> {
    let mut result = Vec::new();
    if let (Some(root1), Some(root2)) = (&tree1.root, &tree2.root) {
        compare_nodes_merkle(root1, root2, &mut result);
    }
    result
}

fn compare_nodes_merkle(node1: &Node, node2: &Node, result: &mut Vec<u32>) {
    if node1.hash == node2.hash {
        // If nodes match, their leaves match
        for _ in 0..node1.num_leaves {
            result.push(0);
        }
    } else {
        // If nodes do not match, compare their children
        match (&node1.left, &node1.right, &node2.left, &node2.right) {
            (Some(left1), Some(right1), Some(left2), Some(right2)) => {
                compare_nodes_merkle(left1, left2, result);
                compare_nodes_merkle(right1, right2, result);
            }
            (Some(left1), None, Some(left2), None) => {
                compare_nodes_merkle(left1, left2, result);
            }
            (None, Some(right1), None, Some(right2)) => {
                compare_nodes_merkle(right1, right2, result);
            }
            _ => {
                // Handle cases where tree structures do not match
                for _ in 0..node1.num_leaves.max(node2.num_leaves) {
                    result.push(1);
                }
            }
        }
    }
}
//-------------------------------------------------------------------- MERKLE TREE COMPARISON: END --------------------------------------------------------------------


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

pub fn build_tree(leaves_original: Vec<String>) -> MerkleTree {
    let leaves_as_str_original: Vec<&str> = leaves_original.iter().map(|s| s.as_str()).collect();
    let merkle_tree = MerkleTree::new(leaves_as_str_original.clone());
    merkle_tree
}


