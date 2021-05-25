use slc::devices::OutputDevice;
use std::{
    io::Write,
    net::TcpStream,
    thread,
    time::{Duration, Instant},
};

const IP: &str = "192.168.1.235:11000";
const SEND_TIMING: f32 = 1.0 / 240.0;

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
            let start = Instant::now();
            let mut last = 0.0;

            loop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < SEND_TIMING {
                    continue;
                };

                thread::sleep(Duration::from_millis(1));
                let read = controller.read().unwrap();
                let mut buffer = [0; 660 * 4];
                let mut count = 0;
                for led in read.room.leds() {
                    if count >= 660 * 4 {
                        break;
                    }
                    //println!("sending led: {:?}", led);
                    buffer[count] = 0;
                    buffer[count + 1] = led.0;
                    buffer[count + 2] = led.1;
                    buffer[count + 3] = led.2;
                    count += 4;
                    //stream.write(&[0, led.0, led.1, led.2]).unwrap();
                }
                stream.write(&buffer).unwrap();
                stream.write(&[1, 0, 0, 0]).unwrap();
                stream.write(&[2, read.room.brightness, 0, 0]).unwrap();
                drop(read);
                last = duration;
            }
        } else {
            println!("couldn't connect to the server...");
        }
    }
}
