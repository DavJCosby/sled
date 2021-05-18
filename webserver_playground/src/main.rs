// CLIENT

use slc::prelude::*;

use slc_gui::Gui;
use slc_lab_rainbow::*;
use slc_net_client::Client;

pub fn main() {
    let room = Room::new_from_file("../room_configs/myroom.rcfg");
    // create a room_controller with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = Rainbow::new();
    let output = Client::new();

    input.start(rc_input_handle);
    output.start(rc_output_handle);
}