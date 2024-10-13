use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    previous_hash: u64,
    hash: u64,
    data: String,
}

impl Block {
    fn new(index: u64, previous_hash: u64, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(index, timestamp, previous_hash, &data);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, previous_hash: u64, data: &str) -> u64 {
        let mut hash = index + previous_hash;
        for byte in data.bytes() {
            hash += byte as u64;
        }
        hash += timestamp as u64;
        hash
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
        }
    }

    // Origin block
    fn create_genesis_block() -> Block {
        Block::new(0, 0, String::from("Genesis Block"))
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash, data);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                current_block.previous_hash,
                &current_block.data,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    fn print(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}

fn main() {

    /*
    let mut block0 :Block = Block::new(0, 0, String::from("block0"));
    let mut block1 :Block = Block::new(1, 1, String::from("block1"));
    let mut block2 :Block = Block::new(2, 2, String::from("block2"));
    let mut block3 :Block = Block::new(3, 3, String::from("block3"));
    println!("block0: {}", block0.hash);
    println!("block1: {}", block1.hash);
    println!("block2: {}", block2.hash);
    println!("block3: {}", block3.hash);
    */

    let mut blockchain = Blockchain::new();

    blockchain.add_block("Block 1 - MORE 10 BTC".to_string());
    blockchain.add_block("block 2 - LESS 5 BTC".to_string());

    blockchain.print();
    
    println!("Is Blockchain valid ? {}", blockchain.is_valid());

}