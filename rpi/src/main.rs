use rs_ws281x::*;
use slc::prelude::*;
use std::time::Instant;

const REFRESH_TIMING: f32 = 1.0 / 120.0;

struct GPIOOutput;

impl GPIOOutput {
    pub fn new() -> Self {
        GPIOOutput
    }
}

impl OutputDevice for GPIOOutput {
    fn start(&self, controller: std::sync::Arc<std::sync::RwLock<slc::prelude::RoomController>>) {
        println!("hello world");
        let mut controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                0,
                ChannelBuilder::new()
                    .pin(18)
                    .count(60)
                    .strip_type(StripType::Ws2811Rgb)
                    .brightness(255)
                    .build(),
            )
            .build()
            .unwrap();

        let start = Instant::now();
        let mut last = 0.0;
        loop {
            let duration = start.elapsed().as_secs_f32();

            if duration - last < REFRESH_TIMING {
                continue;
            }

            let r = (duration.sin() * 255.0) as u8;
            let b = (duration.cos() * 255.0) as u8;

            let leds = controller.leds_mut(0);

            leds[0] = [255, 0, 0, 0];
            leds[1] = [0, 255, 0, 0];
            leds[2] = [0, 0, 255, 0];
            leds[3] = [b, r, 0, 0];

            last = duration;

            controller.render();
        }
    }
}

use slc::prelude::*;

use slc_lab_rainbow::Rainbow;

pub fn main() {
    let room = Room::new_from_file("../room_configs/1mstrip.rcfg");
    // create a room_controller with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = Rainbow::new();
    let output = GPIOOutput::new();

    input.start(rc_input_handle);
    output.start(rc_output_handle);
}
