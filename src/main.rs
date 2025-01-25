#![allow(unused_imports)]
#![allow(dead_code)]
mod block;
mod blockchain;
mod transaction;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::transaction::Transactions;

fn main() {
    let transaction0 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction1 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction2 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let transaction3 = Transaction::new("bob".into(), "jenny".into(), 1023.67);
    let mut transactions = Transactions::new();
    transactions.add(transaction0);
    transactions.add(transaction1);
    transactions.add(transaction2);
    transactions.add(transaction3);

    let mut blockchain = Blockchain::new(String::from("oxidize"));
    blockchain.new_block(transactions.serialize());
    blockchain.new_block(transactions.serialize());
    let decerial: Transactions = Transactions::deserialize(transactions.serialize());
    println!("{:?}", decerial);
    // blockchain.print_blocks();
}
