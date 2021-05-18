use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

const IP: &str = "127.0.0.1:11000";

fn handle_client(stream: TcpStream) {
    println!("got new client!");
    start_receive_loop(stream);
}

fn start_receive_loop(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 4];
        let read_result = stream.read(&mut buffer);
        match read_result {
            Ok(0) => {
                println!("Connection closed.");
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(_) => { /* success; do nothing */ }
        }
        println!("Got color: ({}, {}, {})", buffer[1], buffer[2], buffer[3]);
    }
}

fn process_byte_chunk(chunk: [u8; 4]) {
    match chunk[0] {
        
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(IP)?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
