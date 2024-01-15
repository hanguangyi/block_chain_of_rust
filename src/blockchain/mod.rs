use self::block::Block;
use std::error::Error;

mod block;
#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>,
}

impl Blockchain{
    pub fn add_block(&mut self, data: String) -> Result<(), Box<dyn Error>>{
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new(data, prev_block.hash.clone())?;
        self.blocks.push(new_block);
        Ok(())
    }
    pub fn new() -> Result<Blockchain, Box<dyn Error>> {
        let mut blockchain = Blockchain {
            blocks: vec![Block::new_genesis()?],
        };
        Ok(blockchain)
    }
}

