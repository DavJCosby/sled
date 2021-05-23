use std::{
    io::Read,
    sync::{Arc, RwLock},
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use slc::prelude::*;

const IP: &str = "192.168.1.238:11000";

pub struct Server {
    stop: bool,
}

impl InputDevice for Server {
    fn start(self, controller: Arc<RwLock<RoomController>>) {
        thread::spawn(move || {
            let listener = TcpListener::bind(IP).unwrap();

            for stream in listener.incoming() {
                self.handle_client(stream.unwrap(), Arc::clone(&controller));
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}

impl Server {
    pub fn new() -> Server {
        Server { stop: false }
    }

    fn handle_client(&self, mut stream: TcpStream, controller_handle: Arc<RwLock<RoomController>>) {
        println!("got new client!");
        let mut led_index = 0;
        while !self.stop {
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
            //println!("Got color: ({}, {}, {})", buffer[1], buffer[2], buffer[3]);

            match buffer[0] {
                0 => {
                    let mut write = controller_handle.write().unwrap();
                    write.set(led_index, (buffer[1], buffer[2], buffer[3]));
                    drop(write);
                    led_index += 1;
                }
                1 => {
                    /* new frame */
                    led_index = 0;
                }
                x => {
                    println!("unexpected identifier: {}", x);
                }
            }
        }
    }
}
