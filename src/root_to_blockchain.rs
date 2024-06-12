use std::time::{SystemTime, UNIX_EPOCH};
use rs_merkle::{MerkleTree, algorithms::Sha256, Hasher};

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

 impl  Blockchain {
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
    pub fn add_block(&mut self,merkle_root:String,transctions:Vec<String>){
        let prev_block = self.chain.last().unwrap();
        let prev_blockhash = calculate_hash(&prev_block.header);

        let new_block = Block{
            header:Header{
                version:1,
                prev_blockhash,
                merkle_root,
                time:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32,
                nonce:0
            },
           transaction:Transaction{
            tx:transctions,
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

pub fn calculate_root(leaves: Vec<String>) {
    let mut blockchain = Blockchain::new();

    let leaves_as_bytes: Vec<[u8; 32]> = leaves.iter().map(|x| Sha256::hash(x.as_bytes())).collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves_as_bytes);

    match merkle_tree.root_hex() {
        Some(root) => {
            println!("This is the root node: {:?}", root);
            blockchain.add_block(root, leaves)
            
        }
        None => eprintln!("Couldn't get the merkle root"),
    }

    blockchain.print_blockchain();
}

