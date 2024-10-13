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

fn main() {

    let mut block0 :Block = Block::new(0, 0, String::from("block0"));
    let mut block1 :Block = Block::new(1, 1, String::from("block1"));
    let mut block2 :Block = Block::new(2, 2, String::from("block2"));
    let mut block3 :Block = Block::new(3, 3, String::from("block3"));


    println!("block0: {}", block0.hash);
    println!("block1: {}", block1.hash);
    println!("block2: {}", block2.hash);
    println!("block3: {}", block3.hash);

}