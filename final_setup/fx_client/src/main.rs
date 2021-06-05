// CLIENT
use std::{thread, time::Duration};

use calibration::Calibration;
use lab_rainbow::Rainbow;
use networking::Client;
use slc::prelude::*;
use sweep::Sweep;
use warpspeed::Warpspeed;

// new
pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");
    room.set_input_device(Calibration::new());
    room.add_output_device(Client::new());

    room.start();
    thread::sleep(Duration::from_secs(10));
    room.stop();
}
