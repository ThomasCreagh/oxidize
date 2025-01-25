use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
pub struct Block {
    timestamp: u128,
    hash: Vec<u8>,
    previous_hash: Vec<u8>,
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>, previous_hash: Vec<u8>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time error")
            .as_millis();
        let mut new_block = Block {
            timestamp,
            hash: Vec::new(),
            previous_hash,
            data,
        };
        new_block.hash();
        new_block
    }

    pub fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn get_timestamp(&self) -> u128 {
        self.timestamp.clone()
    }

    fn hash(&mut self) {
        let seriarlized = serde_json::to_vec(&self).unwrap();
        let result = blake3::hash(&seriarlized);
        self.hash = result.to_string().into_bytes();
    }
}
