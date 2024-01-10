mod block;
pub struct Blockchain{
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain>{
        let mut blockchain = Blockchain{
            blocks: Vec![Block::new_genesis()?],
        };
        Ok(blockchain)
    }
    
    pub fn add_block(&mut self, data: String) -> Result<()>{
        let prev_hash = self.blocks.last().unwrap().hash.clone();
        let block = Block::new(data, prev_hash)?;
        self.blocks.push(block);
        Ok(())
    }
}
