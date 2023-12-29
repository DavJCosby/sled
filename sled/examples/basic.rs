mod tui;
use std::f32::consts::TAU;

use tui::SledTerminalDisplay;

use sled::driver::{Driver, Filters, Sliders, TimeInfo};
use sled::{color::Rgb, scheduler::Scheduler, Sled, SledError};

fn startup(sled: &mut Sled, sliders: &mut Sliders, filters: &mut Filters) -> Result<(), SledError> {
    sliders.set("background", Rgb::new(0.0, 0.0, 0.0));
    sliders.set("light_color", Rgb::new(1.0, 1.0, 1.0));

    filters.set("left_wall", sled.get_segment(2).unwrap());
    filters.set("cone", sled.filter_by_angle(|a| a > 0.2 && a <= 0.6));
    Ok(())
}

const GREEN_RADIUS: f32 = 35.0;
const GREEN_COUNT: usize = 64;

const BLUE_RADIUS: f32 = 45.0;
const BLUE_COUNT: usize = 96;

const TRAIL_RADIUS: f32 = 18.0;

fn draw(
    sled: &mut Sled,
    _sliders: &Sliders,
    _filters: &Filters,
    time_info: &TimeInfo,
) -> Result<(), SledError> {
    let elapsed = time_info.elapsed.as_secs_f32() * 15.0;
    let inner_color = Rgb::new(0.6, 0.93, 0.762);
    let outer_delta = Rgb::new(0.4, 0.51, 0.93);

    let inner_time_scale = elapsed / GREEN_RADIUS;
    let outer_time_scale = elapsed / BLUE_RADIUS;

    for i in 0..GREEN_COUNT {
        let angle = inner_time_scale + (TAU / GREEN_COUNT as f32) * i as f32;
        sled.modulate_at_angle(angle, |led| led.color + inner_color)
            .unwrap();
    }

    for i in 0..BLUE_COUNT {
        let angle = outer_time_scale + (TAU / BLUE_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + outer_delta)
            .unwrap();
    }

    let radar_time_scale = elapsed / TRAIL_RADIUS;
    let angle = radar_time_scale % TAU;
    sled.map(|led| {
        let da = (led.angle() + angle) % TAU;
        let fac = 1.0 - (da / (TAU)).powf(1.25);
        led.color * fac
    });

    Ok(())
}

// fn draw(
//     sled: &mut Sled,
//     sliders: &Sliders,
//     filters: &Filters,
//     time_info: &TimeInfo,
// ) -> Result<(), SledError> {
//     let bg_color = sliders.get("background").ok_or("bg_color not found")?;
//     let light_color = sliders.get("light_color").ok_or("light_color not found")?;

//     let cone = filters.get("cone").ok_or("cone not found")?;
//     let left_wall = filters.get("left_wall").ok_or("left_wall not found")?;

//     let peak_br = (time_info.elapsed.as_secs_f32() / 20.0).sin() + 1.0;

//     sled.set_all(bg_color);
//     sled.set_filter(cone, light_color);
//     sled.modulate_filter(left_wall, |led| {
//         let d_sq = led.distance().powi(2);
//         light_color * peak_br / d_sq
//     });

//     Ok(())
// }

fn main() {
    let sled = Sled::new("./examples/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Sled Visualizer", sled.domain());

    let mut driver = Driver::new();

    driver.set_startup_commands(startup);
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
