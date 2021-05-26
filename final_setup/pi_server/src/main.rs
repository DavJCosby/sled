pub mod gpio_output;

use crate::gpio_output::GPIOOutput;

use slc::prelude::*;
use networking::Server;

pub fn main() {
    let room = Room::new_from_file("../../room_configs/myroom.rcfg");
    // create a room_controller with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = Server::new();
    let output = GPIOOutput::new();
    // start input and output devices
    input.start(rc_input_handle);
    output.start(rc_output_handle);
}
