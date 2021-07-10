// CLIENT
use networking::Client;
use std::{thread, time::Duration};
#[allow(unused_imports)]
use {
    audio_visualizer::AudioVisualizer, calibration::Calibration, lab_rainbow::Rainbow,
    slc::prelude::*, sweep::Sweep, warpspeed::Warpspeed, light_painting::Canvas
};

// new
pub fn main() {
    let mut room = Room::new("../../room_configs/myroom.rcfg");

    //let input = Warpspeed::new((-0.00062, 1.0), 0.5);
    //let input = Rainbow::new(1.0, 1.2);
    //let input = Sweep::new(0.8);
    let input = Canvas::new();
    //let input = Calibration::new();
    //let input = AudioVisualizer;
    let output = Client::new("192.168.1.234:11000");

    room.set_input_device(input);
    room.add_output_device(output);

    room.start();
    thread::sleep(Duration::from_secs(10_000));
    room.stop();
}
