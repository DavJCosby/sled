// CLIENT
use std::{thread, time::Duration};

use calibration::Calibration;
use lab_rainbow::Rainbow;
use slc::prelude::*;
use sweep::Sweep;
use warpspeed::Warpspeed;

pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");
    room.set_input(Warpspeed::new((-0.62, 1.0), 1.0));
    room.add_output(networking::Client::new());
    room.start();
    println!("started");
    thread::sleep(Duration::from_secs(10));
    println!("stopping");
    room.stop();
}
