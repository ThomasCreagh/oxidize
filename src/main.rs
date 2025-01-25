mod block;
mod transaction;

use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transaction = Transaction {
        sender: "a".into(),
        reciever: "b".into(),
        amount: 32.1,
    };
    let mut transactions: Vec<Transaction> = Vec::new();
    transactions.push(transaction);

    let block = Block::new(transactions, Vec::new());

    print!("{:#?}", block);
}
