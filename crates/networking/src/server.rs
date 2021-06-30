use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
    thread,
};

use slc::prelude::*;

const BUFFER_SIZE: usize = 128;
pub struct Server {
    ip: SocketAddr,
}

impl InputDevice for Server {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        let ip_ref = Arc::new(self.ip);
        thread::spawn(move || {
            let listener = TcpListener::bind(*ip_ref).unwrap();

            for stream in listener.incoming() {
                handle_client(stream.unwrap(), input_handle.clone());
            }
        });
    }

    fn stop(&mut self) {}
}

fn handle_client(mut stream: TcpStream, input_handle: RoomControllerInputHandle) {
    println!("got new client!");

    let mut led_index = 0;
    let mut local_stop = false;
    while !(local_stop) {
        let mut buffer = [0; BUFFER_SIZE];
        let read_result = stream.read_exact(&mut buffer);

        match read_result {
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(_) => { /* success; do nothing */ }
        }

        for chunk in buffer.chunks(4) {
            let op = chunk[0];
            let x = chunk[1];
            let y = chunk[2];
            let z = chunk[3];

            match op {
                0 => {
                    let mut write = input_handle.write().unwrap();
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
                    let mut write = input_handle.write().unwrap();
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
    }
}

impl Server {
    pub fn new(ip: &str) -> Server {
        Server {
            ip: ip.parse().unwrap(),
        }
    }
}
