use slc::devices::OutputDevice;
use std::{io::Write, net::TcpStream, thread, time::Duration};

const IP: &str = "127.0.0.1:11000";

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Client
    }
}

impl OutputDevice for Client {
    fn start(&self, controller: std::sync::Arc<std::sync::RwLock<slc::prelude::RoomController>>) {
        if let Ok(mut stream) = TcpStream::connect(IP) {
            println!("connected to the server!");
            loop {
                thread::sleep(Duration::from_secs(1));
                let read = controller.read().unwrap();
                for led in read.room.leds() {
                    stream.write(&[0, led.0, led.1, led.2]).unwrap();
                }
                drop(read);
                stream.write(&[1, 0, 0, 0]).unwrap();
            }
        } else {
            println!("couldn't connect to the server...");
        }
    }
}
