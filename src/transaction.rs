use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub sender: String,
    pub reciever: String,
    pub amount: f32,
}
