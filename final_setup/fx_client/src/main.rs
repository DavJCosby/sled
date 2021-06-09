// CLIENT
use std::{thread, time::Duration};

use calibration::Calibration;
use lab_rainbow::Rainbow;
use networking::Client;
use slc::prelude::*;
use sweep::Sweep;
use warpspeed::Warpspeed;
use audio_visualizer::AudioVisualizer;

// new
pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");

    let input = Warpspeed::new((-0.00062, 1.0), 0.5);
    //let input = Rainbow::new(1.0, 1.0);
    //let input = Sweep::new(0.8);
    //let input = Calibration::new();
    //let input = AudioVisualizer;
    let output = Client::new();

    room.set_input_device(input);
    room.add_output_device(output);

    room.start();
    thread::sleep(Duration::from_secs(1000));
    room.stop();
}
