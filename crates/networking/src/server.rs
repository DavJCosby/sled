use std::{
    io::Read,
    sync::{Arc, RwLock},
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use slc::prelude::*;

const IP: &str = "192.168.1.234:11000";

pub struct Server {
    stop: bool,
}

impl InputDevice for Server {
    fn start(&self, controller: Arc<RwLock<RoomController>>) {
        let listener = TcpListener::bind(IP).unwrap();

        for stream in listener.incoming() {
            self.handle_client(stream.unwrap(), Arc::clone(&controller));
        }
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
        let stop_watcher = Arc::new(self.stop);
        thread::spawn(move || {
            println!("got new client!");

            let mut led_index = 0;
            let mut local_stop = false;
            while !(*stop_watcher || local_stop) {
                let mut buffer = [0; 4];
                let read_result = stream.read_exact(&mut buffer);
                match read_result {
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                    Ok(_) => { /* success; do nothing */ }
                }

                let op = buffer[0];
                let x = buffer[1];
                let y = buffer[2];
                let z = buffer[3];

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
                    2 => {
                        /* change brightness to x */
                        let mut write = controller_handle.write().unwrap();
                        write.room_data.brightness = x;
                        drop(write);
                    }
                    3 => {
                        /* stop server */
                        local_stop = true;
                    }
                    x => {
                        println!("unexpected identifier: {}", x);
                    }
                }
            }
        });
    }
}
