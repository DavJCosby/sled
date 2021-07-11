pub mod gpio_output;
use crate::gpio_output::GPIOOutput;
use networking::Server;
use slc::prelude::*;
use std::{thread, time::Duration};

pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");

    let input = Server::new("192.168.1.234:11000");
    let output = GPIOOutput::new();

    room.set_input_device(input);
    room.add_output_device(output);

    room.start();
    loop {
        thread::sleep(Duration::from_secs(1_000));
    }
}
