use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha512, Digest};
use hex;

/// Estrutura do bloco
#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    /// Criação de um novo bloco
    pub fn new(index: u64, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &data);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    /// Cálculo seguro do hash usando SHA-512
    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, previous_hash, data);
        let mut hasher = Sha512::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result) // Convertendo para String hexadecimal
    }
}