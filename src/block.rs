use std::time::SystemTime;

use crate::transaction::Transaction;

use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub struct Block {
    timestamp: u128,
    hash: Vec<u8>,
    previous_hash: Vec<u8>,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time error")
            .as_millis();
        let mut new_block = Block {
            timestamp,
            hash: Vec::new(),
            previous_hash,
            transactions,
        };
        new_block.hash();
        return new_block;
    }

    fn hash(&mut self) {
        let seriarlized = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(seriarlized);
        let result = hasher.finalize();
        self.hash = result.to_vec();
    }
}
