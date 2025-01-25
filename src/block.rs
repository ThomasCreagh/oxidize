use std::time::SystemTime;

use crate::transaction::Transaction;

use serde::Serialize;
// use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub struct Block {
    timestamp: u128,
    hash: String,
    previous_hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time error")
            .as_millis();
        let mut new_block = Block {
            timestamp,
            hash: String::new(),
            previous_hash,
            transactions,
        };
        new_block.hash();
        return new_block;
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn hash(&mut self) {
        let seriarlized = serde_json::to_vec(&self).unwrap();
        let result = blake3::hash(&seriarlized);
        self.hash = result.to_string();
    }
}
