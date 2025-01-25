use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
struct Block {
    timestamp: i64,
    hash: Vec<u8>,
    previous_hash: Vec<u8>,
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}

fn main() {
    let transaction = Transaction {
        sender: "a".into(),
        reciever: "b".into(),
        amount: 32.1,
    };
    let mut transactions: Vec<Transaction> = Vec::new();
    transactions.push(transaction);
    let mut new_block = Block {
        timestamp: 2,
        hash: Vec::new(),
        previous_hash: Vec::new(),
        transactions: transactions,
    };

    let seriarlized = serde_json::to_string(&new_block).unwrap();

    let mut hasher = Sha256::new();

    hasher.update(seriarlized);

    let result = hasher.finalize();

    new_block.hash = result.to_vec();

    print!("{:#?}", new_block);
}
