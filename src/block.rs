use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: u64,
    pub hash: u64,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: u64, data: String) -> Block {
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

    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: u64, data: &str) -> u64 {
        let mut hash = index + previous_hash;
        for byte in data.bytes() {
            hash += byte as u64;
        }
        hash += timestamp as u64;
        hash
    }
}
