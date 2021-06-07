use std::{thread, time::Duration};

use slc::prelude::*;

use calibration::Calibration;
use lab_rainbow::Rainbow;
use slc_gui::Gui;
use sweep::Sweep;
//use warpspeed::Warpspeed;

pub fn main() {
    let mut room = Room::new("../room_configs/myroom.rcfg");
    //let input = Warpspeed::new((-0.62, 1.0), 1.0);
    let input = Rainbow::new(1.0, 1.0);
    let output = Gui::new();
    room.set_input_device(input);
    room.add_output_device(output);
    room.start();
    thread::sleep(Duration::from_secs(1));
    room.stop();
}
