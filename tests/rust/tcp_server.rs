use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let message = String::from_utf8_lossy(&buffer).to_string();
        println!("Received message: {}", message);

        stream.write(message.as_bytes()).unwrap();
    }
}
