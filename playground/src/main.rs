use std::sync::{Arc, RwLock};

use slc::{
    devices::{InputDevice, OutputDevice},
    room::Room,
    room_controller::RoomController,
};

use slc_gui::Gui;
use slc_sweep::Sweep;

pub fn main() {
    let room = Room::new_from_file("../room_configs/config1.rcfg");
    let room_controller = RoomController { room };

    // create a read-write lock on the room_controller for safe multithreaded access
    let input_access = Arc::new(RwLock::new(room_controller));
    let output_access = input_access.clone();

    // prepare input and output devices
    let input = Sweep::new();
    let output = Gui::new();

    input.start(input_access);
    output.start(output_access);
}
