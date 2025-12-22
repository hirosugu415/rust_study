pub mod block;
pub mod blockchain;

use blockchain::Blockchain;
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let bc = Arc::new(Mutex::new(Blockchain::new()));

    let args: Vec<String> = env::args().collect();
    let port = args.get(1).unwrap_or(&"7878".to_string()).clone();
    let address = format!("127.0.0.1:{}", port);

    let listner = TcpListener::bind(&address).unwrap();
    println!("Server listening on {}", address);

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        let bc_clone = Arc::clone(&bc);
        thread::spawn(move || {
            handle_connection(stream, bc_clone);
        });
    }
}

fn handle_connection(mut stream: TcpStream, bc: Arc<Mutex<Blockchain>>) {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf[..]);
    if request.starts_with("POST /register") {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();
        if parts.len() > 1 {
            let body = parts[1].trim_matches('\0');
            // 他のノードのアドレス(例: "127.0.0.1:7879")を登録
            bc.lock().unwrap().register_node(body.to_string());
            println!("Node registered: {}", body);
        }

        stream
            .write_all(b"HTTP/1.1 200 OK\r\n\r\nRegistered")
            .unwrap();
    } else if request.starts_with("POST /add_block") {
        let parts: Vec<&str> = request.split("\r\n\r\n").collect();
        if parts.len() > 1 {
            let body = parts[1].trim_matches('\0');
            bc.lock().unwrap().add_block(body.to_string());
            println!("New block mined!");
        }
        stream
            .write_all(b"HTTP/1.1 200 OK\r\n\r\nBlock Added")
            .unwrap();
    } else if request.starts_with("GET /resolve") {
        bc.lock().unwrap().resolve_conflicts();
    } else {
        let bc_data = bc.lock().unwrap();
        let bc_json = serde_json::to_string_pretty(&*bc_data).unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            bc_json.len(),
            bc_json
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}
