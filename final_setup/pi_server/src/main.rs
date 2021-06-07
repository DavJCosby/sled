pub mod gpio_output;

use crate::gpio_output::GPIOOutput;

use networking::Server;
use slc::prelude::*;

use std::{thread, time::Duration};

pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");
    // create a room_controller with a RwLock for safe multithreading

    // prepare input and output devices
    let input = Server::new();
    let output = GPIOOutput::new();

    room.set_input_device(input);
    room.add_output_device(output);
    println!("starting room...");
    // start input and output devices
    room.start();
    println!("post...");
    thread::sleep(Duration::from_secs(1000));
    room.stop();
}
