use std::{thread::sleep, time::Duration};

use crate::blockchain::Blockchain;
mod blockchain;


fn main() {
    let mut blockchain = Blockchain::new().unwrap();
    sleep(Duration::from_millis(10));
    blockchain.add_block("Send 1 BTC to Ivan".to_string()).unwrap();
    blockchain.add_block("Send 2 more BTC to Ivan".to_string()).unwrap();
    println!("{:#?}", blockchain);
}
