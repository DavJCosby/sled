use core::time;
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

const IP: &str = "127.0.0.1:11000";

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect(IP) {
        println!("Connected to the server!");
        stream.write(&[0, 255, 0, 0])?;
        println!("Sent red pixel!");
        thread::sleep(time::Duration::from_secs(5));
        println!("five seconds have passed, time for a blue pixel.");
        stream.write(&[0, 0, 0, 255])?;
        thread::sleep(time::Duration::from_secs(5));
        println!("five more seconds, time for a green pixel.");
        stream.write(&[0, 0, 255, 0])?;
    } else {
        println!("Couldn't connect to the server...");
    }

    Ok(())
}
