use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
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
