use rs_ws281x::*;
use slc::prelude::*;
use std::time::Instant;
use std::thread;

const REFRESH_TIMING: f32 = 1.0 / 120.0;

struct GPIOOutput;

impl GPIOOutput {
    pub fn new() -> Self {
        GPIOOutput
    }
}

impl OutputDevice for GPIOOutput {
    fn start(&self, rc: std::sync::Arc<std::sync::RwLock<slc::prelude::RoomController>>) {
        thread::spawn(move || {
            let read = rc.read().unwrap();
            let num_leds = read.room.leds().len();

            println!("Booted room with {} leds.", num_leds);
            let mut controller = ControllerBuilder::new()
                .freq(800_000)
                .dma(10)
                .channel(
                    0,
                    ChannelBuilder::new()
                        .pin(18)
                        .count(num_leds as i32)
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

                let leds = controller.leds_mut(0);

                let mut counter = 0;
                for (r, g, b) in read.room.leds() {
                    leds[num_leds - counter - 1] = [*r, *g, *b, 0];
                    counter += 1;
                }

                last = duration;
                controller.render();
            }
        }).join();
    }
}

use slc::prelude::*;

use slc_lab_rainbow::Rainbow;

pub fn main() {
    let room = Room::new_from_file("../room_configs/myroom.rcfg");
    // create a room_controller with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = Rainbow::new();
    let output = GPIOOutput::new();

    input.start(rc_input_handle);
    output.start(rc_output_handle);
}
