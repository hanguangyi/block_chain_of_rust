pub struct Block{
    pub timestamp: u128,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    nonce: u64,
}
const TARGET_HEXS: u8 = 4;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

impl Block{
    pub fn set_hash(&mut self) -> Result<()>{
        self.timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let content = (self.data.clone(), self.prev_hash.clone(), self.timestamp);
        let bytes = serialize(&content)?;
        let mut hasher = Sha256::new();
        hasher.input(&bytes);
        self.hash = hasher.result_str();
        Ok(())
    }

    pub fn new(data: String, prev_hash: String) -> Result<Block>{
        let mut block = Block{
            timestamp: 0,
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.set_hash()?;
        Ok(block)
    }

    pub fn new_genesis() -> Result<Block>{
        let mut block = Block::new("Genesis".to_string(),String::new())?;
        Ok(block)
    }

    pub fn prepare_data(&self) -> Result<String>{
        let content = (
            self.data.clone(), 
            self.prev_hash.clone(), 
            self.timestamp, 
            TARGET_HEXS,
            self.nonce,
        );
        let bytes = serialize(&content)?;
        Ok(bytes)
    }
    pub fn run_pow(&mut self) -> Result<()>{
        println!("Mining the block containing \"{}\"", self.data);
        while !self.validate()?{
            self.nonce += 1;
            }
        let data = self.prepare_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data);
        self.hash = hasher.result_str();
        Ok(())
    }
}