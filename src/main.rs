mod block;
mod transaction;

use crate::block::Block;
use crate::transaction::Transaction;

fn main() {
    let transaction0 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction1 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction2 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction3 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let mut transactions0: Vec<Transaction> = Vec::new();
    transactions0.push(transaction0);
    transactions0.push(transaction1);
    transactions0.push(transaction2);
    transactions0.push(transaction3);

    let transactions1: Vec<Transaction> = transactions0.clone();
    let transactions2: Vec<Transaction> = transactions0.clone();
    let transactions3: Vec<Transaction> = transactions0.clone();

    let block0 = Block::new(transactions0, String::new());
    let block1 = Block::new(transactions1, block0.get_hash());
    let block2 = Block::new(transactions2, block1.get_hash());
    let block3 = Block::new(transactions3, block2.get_hash());

    println!("{:#?}", block0);
    println!("{:#?}", block1);
    println!("{:#?}", block2);
    println!("{:#?}", block3);
}
