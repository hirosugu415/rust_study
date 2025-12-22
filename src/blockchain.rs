use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::block::Block;
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    peers: HashSet<String>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut genesis_block = Block {
            id: 0,
            hash: String::new(),
            previous_hash: String::from("0"),
            timestamp: timestamp,
            data: String::from("Genesis Block"),
            nonce: 0,
        };

        genesis_block.mine_block(2);

        Blockchain {
            chain: vec![genesis_block],
            peers: HashSet::new(),
        }
    }

    pub fn add_block(&mut self, data: String) {
        let last_block = self.chain.last().unwrap();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut new_block = Block {
            id: last_block.id + 1,
            hash: String::new(),
            previous_hash: last_block.hash.clone(),
            data: data,
            timestamp: timestamp,
            nonce: 0,
        };

        new_block.mine_block(2);

        self.chain.push(new_block);
    }

    pub fn is_chain_valid(chain: &Vec<Block>) -> bool {
        for i in 1..chain.len() {
            let current_block = &chain[i];
            let previous_block = &chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>)->bool{
        if new_chain.len() > self.chain.len() {
            if Blockchain::is_chain_valid(&new_chain) {
                self.chain = new_chain;
                return true;
            }
        }
        false
    }

    pub fn register_node(&mut self, address: String) {
        self.peers.insert(address);
    }

    pub fn resolve_conflicts(&mut self)->bool{
        let mut replaced = false;
        for peer in &self.peers {
            let url = format!("http://{}/", peer);
            if let Ok(resp) = reqwest::blocking::get(&url){
                if let Ok(chain) = resp.json::<Vec<Block>>() {
                    if self.replace_chain(chain){
                        replaced = true;
                    }
                }
            }
        }
        replaced
    }
}
