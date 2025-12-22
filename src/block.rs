use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn calculate_hash(&self) -> String {
        let formatted_block_data = format!(
            "{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(formatted_block_data);
        let result = hasher.finalize();

        hex::encode(result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = vec!["0"; difficulty].join("");
        loop {
            let hash = self.calculate_hash();
            if hash.starts_with(&target) {
                self.hash = hash;
                break;
            }
            self.nonce += 1;
        }
    }
}
