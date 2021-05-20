use rs_ws281x::*;
use std::time::Instant;

const REFRESH_TIMING: f32 = 1.0 / 120.0;

fn main() {
    println!("hello world");
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(10)
                .strip_type(StripType::Ws2811Rgb)
                .brightness(255)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    let start = Instant::now();
    let mut last = 0.0;
    loop {
        let duration = start.elapsed().as_secs_f32();

        if duration - last < REFRESH_TIMING {
            continue;
        }

        let r = duration.sin() as u8;
        let b = duration.cos() as u8;

        leds[0] = [255, 0, 0, 0];
        leds[1] = [0, 255, 0, 0];
        leds[2] = [0, 0, 255, 0];
        leds[3] = [b, r, 0, 0];

        last = duration;

        controller.render();
    }
}
