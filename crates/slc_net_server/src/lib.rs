use std::{
    io::Read,
    sync::{Arc, RwLock},
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use slc::prelude::*;

const IP: &str = "192.168.1.235:11000";

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
            let mut buffer = [0; 661 * 4];
            let read_result = stream.read_exact(&mut buffer);
            match read_result {
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                Ok(_) => { /* success; do nothing */ }
            }
            //println!("Got color: ({}, {}, {})", buffer[1], buffer[2], buffer[3]);

            for i in 0..661 {
                let cursor = i * 4;
                let op = buffer[cursor];
                let x = buffer[cursor + 1];
                let y = buffer[cursor + 2];
                let z = buffer[cursor + 3];

                match op {
                    0 => {
                        let mut write = controller_handle.write().unwrap();
                        write.set(led_index, (x, y, z));
                        drop(write);
                        led_index += 1;
                    }
                    1 => {
                        /* new frame */
                        led_index = 0;
                    }
                    2 => { /* change brightness */ }
                    3 => { /* shut down */ }
                    x => {
                        println!("unexpected identifier: {}", x);
                    }
                }
            }
        }
    }
}
