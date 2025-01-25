use crate::block::Block;
use crate::transaction::Transactions;

pub struct Blockchain {
    name: String,
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new(name: String) -> Self {
        let genesis_block = Block::new(
            String::from("This is the genesis block on the oxidize blockchain!").into_bytes(),
            Vec::new(),
        );
        Blockchain {
            name,
            blocks: vec![genesis_block],
        }
    }

    pub fn new_block(&mut self, data: Vec<u8>) {
        let last_block = self.blocks.last().expect("genesis block not created");
        let new_block = Block::new(data, last_block.get_hash());
        self.blocks.push(new_block);
    }

    pub fn print_blocks(&self) {
        println!("{}:", self.name);
        println!("{:#?}", self.blocks);
    }
}
