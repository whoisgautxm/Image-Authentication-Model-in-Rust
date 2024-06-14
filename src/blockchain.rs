// src/blockchain.rs

use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use std::fmt::{self, Debug, Formatter};
use hex;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>, // Make chain public
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
                nonce: 0,
            },
            transaction: Transaction {
                tx: transactions,
            },
        };
        self.chain.push(new_block);
    }

    pub fn print_blockchain(&self) {
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
pub fn return_transction(blockchain:&Blockchain, block_hash: &str) -> Vec<String> {
    for block in &blockchain.chain {
        if calculate_hash(&block.header) == block_hash {
            return block.transaction.tx.clone();
        }
    }
    Vec::new() // Return an empty vector if no block matches
}