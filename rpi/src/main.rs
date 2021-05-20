use rs_ws281x::*;

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
    leds[0] = [255, 0, 0, 0];
    leds[1] = [0, 255, 0, 0];
    leds[2] = [0, 0, 255, 0];
    leds[3] = [255, 255, 0, 0];
    controller.render();
}
