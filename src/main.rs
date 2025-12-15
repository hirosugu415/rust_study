use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
}

impl Block {
    fn calculate_hash(&self)->String{
        let formatted_block_data = format!("{}{}{}{}{}", self.id, self.previous_hash,self.timestamp, self.data, self.nonce);

        let mut hasher = Sha256::new();
        hasher.update(formatted_block_data);
        let result = hasher.finalize();

        hex::encode(result)
    }

    fn 
}

fn main(){
    let genesis_block = Block {
        id: 0,
        hash: String::from("000abc..."),
        previous_hash: String::from(""),
        timestamp: 163400000,
        data: String::from("Genesis Block"),
        nonce: 0,
    };

    let hash = genesis_block.calculate_hash();

    println!("Genesis Block: {}", hash);
}