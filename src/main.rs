use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let formatted_block_data = format!(
            "{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(formatted_block_data);
        let result = hasher.finalize();

        hex::encode(result)
    }

    fn mine_block(&mut self, difficulty: usize) {
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

#[derive(Debug, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
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
        }
    }

    fn add_block(&mut self, data: String) {
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

    fn _is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let bc = Blockchain::new();

    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    print!("Server listening on port 7878...");

    for stream in listner.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream, &bc);
    }
}

fn handle_connection(mut stream: TcpStream, bc: &Blockchain){
    let mut buf = [ 0u8; 1024 ];
    stream.read(&mut buf).unwrap();

    let mut file = File::open("hello.html").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let bc_json = serde_json::to_string(bc).unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        bc_json.len(),
        bc_json
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}