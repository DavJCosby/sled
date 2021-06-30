#[allow(unused_imports)]
use {
    audio_visualizer::AudioVisualizer, calibration::Calibration, lab_rainbow::Rainbow,
    slc::prelude::*, slc_gui::Gui, sweep::Sweep, warpspeed::Warpspeed,
};

pub fn main() {
    let mut room = Room::new("../room_configs/myroom.rcfg");
    let input = Warpspeed::new((-0.62, 1.0), 1.0);
    //let input = Rainbow::new(1.0, 1.0);
    //let input = AudioVisualizer;
    //let input = Sweep::new(0.04);
    let output = Gui::new();

    room.set_input_device(input);
    room.add_output_device(output);
    room.start();
}
