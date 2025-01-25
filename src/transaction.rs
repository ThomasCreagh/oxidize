use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    payer: String,
    payee: String,
    amount: f64,
    id: Uuid,
}

impl Transaction {
    pub fn new(payer: String, payee: String, amount: f64) -> Self {
        let id = Uuid::new_v4();
        Transaction {
            payer,
            payee,
            amount,
            id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transactions {
    data: Vec<Transaction>,
}

impl Transactions {
    pub fn new() -> Self {
        Transactions { data: Vec::new() }
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.data.push(transaction);
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("serialization failed")
    }

    pub fn deserialize(data: Vec<u8>) -> Transactions {
        serde_json::from_slice(&data).unwrap()
    }
}
