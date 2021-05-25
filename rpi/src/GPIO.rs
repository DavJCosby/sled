use rs_ws281x::*;
use slc::prelude::*;
use std::time::Instant;

const REFRESH_TIMING: f32 = 1.0 / 240.0;

pub struct GPIOOutput;

impl GPIOOutput {
    pub fn new() -> Self {
        GPIOOutput
    }
}

impl OutputDevice for GPIOOutput {
    fn start(&self, rc: std::sync::Arc<std::sync::RwLock<slc::prelude::RoomController>>) {
        let read = rc.read().unwrap();
        let num_leds = read.room.leds().len() as i32;
        let brightness = read.room.brightness;
        drop(read);
        let mut gpio_controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                0,
                ChannelBuilder::new()
                    .pin(18)
                    .count(num_leds)
                    .strip_type(StripType::Ws2811Gbr) // Ws2811Grb
                    .brightness(brightness)
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

            let read = rc.read().unwrap();
            gpio_controller.set_brightness(0, read.room.brightness);

            let leds = gpio_controller.leds_mut(0);
            let mut counter = 0;
            for (r, g, b) in read.room.leds() {
                leds[counter] = [*r, *g, *b, 0];
                counter += 1;
            }

            drop(read);
            gpio_controller.render();
            last = duration;
        }
    }
}
