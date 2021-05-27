// CLIENT

use calibration::Calibration;
use lab_rainbow::Rainbow;
use networking::Client;
use slc::prelude::*;
use sweep::Sweep;
use warpspeed::Warpspeed;

pub fn main() {
    let room = Room::new_from_file("../../room_configs/myroom.rcfg");
    // create a room_controller with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = Warpspeed::new((0.625, 1.0), 0.33);
    let output = Client::new();

    input.start(rc_input_handle);
    output.start(rc_output_handle);
}
