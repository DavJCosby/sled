use std::{f32::consts::TAU, time::Duration};

use sled::driver::{Driver, Filters, Sliders, TimeInfo};
use sled::{color::Rgb, Sled, SledError};

const GREEN_RADIUS: f32 = 2.33;
const GREEN_COUNT: usize = 64;
const GREEN: Rgb = Rgb::new(0.6, 0.93, 0.762);

const BLUE_RADIUS: f32 = 3.0;
const BLUE_COUNT: usize = 96;
const BLUE: Rgb = Rgb::new(0.4, 0.51, 0.93);

const TRAIL_RADIUS: f32 = 1.2;

fn draw(
    sled: &mut Sled,
    _sliders: &Sliders,
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

fn trail(c: &mut Criterion) {
    let sled = Sled::new("./benches/config1.toml").unwrap();
    let mut driver = Driver::new();
    driver.set_draw_commands(draw);
    driver.mount(sled);

    let simulated_duration = 30.0;
    let simulated_hz = 144.0;
    let total_steps = (simulated_duration * simulated_hz) as usize;
    let timestep = Duration::from_secs_f32(1.0 / simulated_hz);
    c.bench_function("quirky_trail", |b| {
        b.iter(|| {
            for _ in 0..total_steps {
                driver.step_by(timestep);
            }
        })
    });
}

use criterion::{criterion_group, criterion_main, Criterion};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .significance_level(0.05)
        .sample_size(500)
        .warm_up_time(Duration::from_secs_f32(10.0))
        .measurement_time(Duration::from_secs_f32(45.0));
    targets = trail
}
criterion_main!(benches);
