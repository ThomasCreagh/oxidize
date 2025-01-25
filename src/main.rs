mod block;
mod transaction;

use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transaction = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let mut transactions: Vec<Transaction> = Vec::new();
    transactions.push(transaction);

    let block = Block::new(transactions, Vec::new());

    print!("{:#?}", block);
}
