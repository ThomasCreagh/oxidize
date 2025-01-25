use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Serialize)]
pub struct Block<T> {
    timestamp: u128,
    hash: String,
    previous_hash: String,
    data: Vec<T>,
}

impl<T: Serialize> Block<T> {
    pub fn new(data: Vec<T>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time error")
            .as_millis();
        let mut new_block = Block::<T> {
            timestamp,
            hash: String::new(),
            previous_hash,
            data,
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
