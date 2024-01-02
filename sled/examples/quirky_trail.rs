mod tui;
use std::f32::consts::TAU;

use tui::SledTerminalDisplay;

use sled::driver::{BufferContainer, Driver, Filters, TimeInfo};
use sled::{color::Rgb, scheduler::Scheduler, Sled, SledError};

const GREEN_RADIUS: f32 = 2.33;
const GREEN_COUNT: usize = 64;
const GREEN: Rgb = Rgb::new(0.6, 0.93, 0.762);

const BLUE_RADIUS: f32 = 3.0;
const BLUE_COUNT: usize = 96;
const BLUE: Rgb = Rgb::new(0.4, 0.51, 0.93);

const TRAIL_RADIUS: f32 = 1.2;

fn draw(
    sled: &mut Sled,
    _sliders: &BufferContainer,
    _filters: &Filters,
    time_info: &TimeInfo,
) -> Result<(), SledError> {
    let elapsed = time_info.elapsed.as_secs_f32();

    let inner_time_scale = elapsed / GREEN_RADIUS;
    let outer_time_scale = elapsed / BLUE_RADIUS;

    // speckle in swirling green points
    for i in 0..GREEN_COUNT {
        let angle = inner_time_scale + (TAU / GREEN_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + GREEN)?
    }

    // speckle in swirling blue points
    for i in 0..BLUE_COUNT {
        let angle = outer_time_scale + (TAU / BLUE_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + BLUE)?
    }

    // brighten or darken points depending on time and angle to simulate a sweeping
    // trail thing.
    let radar_time_scale = elapsed / TRAIL_RADIUS;
    let angle = radar_time_scale % TAU;
    sled.map(|led| {
        let da = (led.angle() + angle) % TAU;
        let fac = 1.0 - (da / (TAU)).powf(1.25);
        led.color * fac
    });

    Ok(())
}

fn main() {
    let sled = Sled::new("./examples/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Sled Visualizer", sled.domain());

    let mut driver = Driver::new();

    driver.set_draw_commands(draw);
    driver.mount(sled);

    let mut scheduler = Scheduler::fixed_hz(500.0);
    scheduler.loop_until_err(|| {
        driver.step();

        display.leds = driver.read_colors_and_positions();
        display.refresh()?;
        Ok(())
    });
}
