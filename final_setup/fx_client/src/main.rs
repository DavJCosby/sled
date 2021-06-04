// CLIENT

use calibration::Calibration;
use lab_rainbow::Rainbow;
use slc::prelude::*;
use sweep::Sweep;
use warpspeed::Warpspeed;

pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");
    room.set_input(Rainbow::new(1.0, 1.0));
    room.add_output(networking::Client::new());
    room.start();
}
