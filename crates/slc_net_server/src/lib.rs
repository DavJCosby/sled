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
const EXPECTED_LEDS: usize = 661;

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
        let mut super_buffer: Vec<u8> = vec![];

        while !self.stop {
            while super_buffer.len() < 128 {
                let mut buffer = [0; 128];
                let read_result = stream.read(&mut buffer);
                let bytes_read = match read_result {
                    Ok(0) => {
                        println!("Connection closed.");
                        break;
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                    Ok(x) => x,
                };

                for i in 0..bytes_read {
                    super_buffer.push(buffer[i]);
                }
            }

            for i in 0..(128 / 4) {
                let si = i * 4;
                let op = super_buffer[si];
                let x = super_buffer[si + 1];
                let y = super_buffer[si + 2];
                let z = super_buffer[si + 3];

                match op {
                    0 => {
                        let mut write = controller_handle.write().unwrap();
                        write.set(led_index % 660, (x, y, z));
                        drop(write);
                        led_index += 1;
                    }
                    1 => {
                        /* new frame */
                        led_index = 0;
                    }
                    x => {
                        println!("unexpected op code: {}", x);
                    }
                }
            }

            super_buffer.drain(0..(128/4));
        }
    }
}
