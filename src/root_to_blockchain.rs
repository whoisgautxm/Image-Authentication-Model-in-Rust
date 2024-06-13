use std::time::{SystemTime, UNIX_EPOCH};
use rs_merkle::{MerkleTree as RsMerkleTree, algorithms::Sha256 as RsSha256, Hasher};
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use hex;

#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: Header,
    pub transaction: Transaction,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub version: u32,
    pub prev_blockhash: String, 
    pub merkle_root: String,
    pub time: u32,
    pub nonce: u32,
}

#[derive(Debug, Clone)]
pub struct Transaction { 
    pub tx: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block {
            header: Header {
                version: 1,
                prev_blockhash: "0".to_string(), 
                merkle_root: "0".to_string(), 
                time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
                nonce: 0,
            },
            transaction: Transaction { 
                tx: vec![],
            },
        };
        Blockchain {
            chain: vec![genesis_block],
        }
    }
    
    pub fn add_block(&mut self, merkle_root: String, transactions: Vec<String>) {
        let prev_block = self.chain.last().unwrap();
        let prev_blockhash = calculate_hash(&prev_block.header);

        let new_block = Block {
            header: Header {
                version: 1,
                prev_blockhash,
                merkle_root,
                time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
                nonce: 0
            },
            transaction: Transaction {
                tx: transactions,
            }
        };
        self.chain.push(new_block);
    }

    fn print_blockchain(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}

pub fn calculate_hash(header: &Header) -> String {
    // Simple hash function for example purposes
    let header_string = format!(
        "{}{}{}{}{}",
        header.version, header.prev_blockhash, header.merkle_root, header.time, header.nonce
    );
    format!("{:x}", md5::compute(header_string))
}

#[derive(Clone)]
struct Node {
    hash: Vec<u8>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(hash: Vec<u8>) -> Self {
        Node {
            hash,
            left: None,
            right: None,
        }
    }

    fn print(&self, depth: usize) {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{:?}", self.hash);

        if let Some(ref left) = self.left {
            left.print(depth + 1);
        }
        if let Some(ref right) = self.right {
            right.print(depth + 1);
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.hash)
    }
}

struct MerkleTree {
    root: Option<Node>,
}

impl MerkleTree {
    fn new(data: Vec<&str>) -> Self {
        let leaves: Vec<Node> = data
            .into_iter()
            .map(|datum| {
                let mut hasher = Sha256::new();
                hasher.update(datum);
                let hash = hasher.finalize().to_vec();
                Node::new(hash)
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

                let mut parent_node = Node::new(parent_hash);
                parent_node.left = Some(Box::new(left));
                parent_node.right = Some(Box::new(right));

                next_level.push(parent_node);
            }

            nodes = next_level;
        }

        nodes.into_iter().next()
    }

    fn print_tree(&self) {
        if let Some(ref root) = self.root {
            root.print(0);
        }
    }

    fn traverse(&self, callback: &mut dyn FnMut(&Node)) {
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

    fn root_hex(&self) -> Option<String> {
        self.root.as_ref().map(|node| hex::encode(&node.hash))
    }
}

pub fn calculate_root(leaves: Vec<String>) {
    let mut blockchain = Blockchain::new();

    let leaves_as_str: Vec<&str> = leaves.iter().map(|s| s.as_str()).collect();
    let merkle_tree = MerkleTree::new(leaves_as_str);

    match merkle_tree.root_hex() {
        Some(root) => {
            println!("This is the root node: {:?}", root);
            merkle_tree.print_tree();
            merkle_tree.traverse(&mut |node| println!("{:?}", node.hash));
            blockchain.add_block(root, leaves)
        }
        None => eprintln!("Couldn't get the merkle root"),
    }

    blockchain.print_blockchain();
}
