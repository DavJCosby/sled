use sled::driver_macros::*;
use sled::driver::{Driver, TimeInfo};
use sled::SledResult;
use sled::{color::Rgb, Sled};

use std::f32::consts::TAU;
const INV_TAU: f32 = 1.0 / TAU;

const GREEN_RADIUS: f32 = 2.33;
const GREEN_COUNT: usize = 64;
const GREEN: Rgb = Rgb::new(0.6, 0.93, 0.762);

const BLUE_RADIUS: f32 = 3.0;
const BLUE_COUNT: usize = 96;
const BLUE: Rgb = Rgb::new(0.4, 0.51, 0.93);

const TRAIL_RADIUS: f32 = 1.2;

#[allow(dead_code)]
pub fn build_driver() -> Driver {
    let mut driver = Driver::new();
    driver.set_draw_commands(draw);
    driver
}

#[draw_commands]
fn draw(sled: &mut Sled, time_info: &TimeInfo) -> SledResult {
    let elapsed = time_info.elapsed.as_secs_f32();

    let inner_time_scale = elapsed / GREEN_RADIUS;
    let outer_time_scale = elapsed / BLUE_RADIUS;

    // speckle in swirling green points
    for i in 0..GREEN_COUNT {
        let angle = inner_time_scale + (TAU / GREEN_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + GREEN);
    }

    // speckle in swirling blue points
    for i in 0..BLUE_COUNT {
        let angle = outer_time_scale + (TAU / BLUE_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + BLUE);
    }

    // brighten or darken points depending on time and angle to simulate a sweeping
    // trail thing.
    let radar_time_scale = elapsed / TRAIL_RADIUS;
    let angle = (radar_time_scale % TAU) + TAU;
    sled.map(|led| {
        let da = (led.angle() + angle) % TAU;
        let fac = 1.0 - (da * INV_TAU).powf(1.25);
        led.color * fac
    });

    Ok(())
}
