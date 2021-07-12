use slc::devices::*;
use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const SEND_TIMING: f32 = 1.0 / 250.0;

pub struct Client {
    ip: SocketAddr,
    connected: Arc<Mutex<bool>>,
}

impl Client {
    pub fn new(ip: &str) -> Self {
        Client {
            ip: ip.parse().unwrap(),
            connected: Arc::new(Mutex::new(false)),
        }
    }
}

impl OutputDevice for Client {
    fn start(&self, output_handle: RoomControllerOutputHandle) {
        let ip_ref = Arc::new(self.ip);
        let connected = self.connected.clone();
        if *connected.lock().unwrap() == true {
            return;
        }
        thread::spawn(move || {
            if let Ok(mut stream) = TcpStream::connect(*ip_ref) {
                println!("connected to the server!");
                let mut guard = connected.lock().unwrap();
                *guard = true;
                drop(guard);

                let start = Instant::now();
                let mut last = 0.0;
                let mut last_brightness = 255;
                loop {
                    let duration = start.elapsed().as_secs_f32();
                    if duration - last < SEND_TIMING {
                        continue;
                    };

                    thread::sleep(Duration::from_millis(1));
                    let read = output_handle.read().unwrap();
                    let mut buffer = [0; 660 * 4];
                    let mut count = 0;
                    for led in read.room_data.leds() {
                        if count >= 660 * 4 {
                            break;
                        }
                        buffer[count] = 0;
                        buffer[count + 1] = led.0;
                        buffer[count + 2] = led.1;
                        buffer[count + 3] = led.2;
                        count += 4;
                    }

                    // write colors
                    match stream.write(&buffer) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("disconnected");
                            *connected.lock().unwrap() = false;
                            return;
                        }
                    }
                    // write end of frame marker
                    stream.write(&[1, 0, 0, 0]).unwrap();

                    // write new brightness, if necessary
                    let current_brightness = read.room_data.brightness;
                    if current_brightness != last_brightness {
                        stream.write(&[2, current_brightness, 0, 0]).unwrap();
                        last_brightness = current_brightness;
                    }

                    last = duration;
                }
            } else {
                println!("couldn't connect to the server...");
            }
        });
    }
}
