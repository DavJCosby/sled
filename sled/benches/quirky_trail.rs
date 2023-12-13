use std::{f32::consts::TAU, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use sled::{Rgb, Sled};

const GREEN_RADIUS: f32 = 35.0;
const GREEN_COUNT: usize = 64;

const BLUE_RADIUS: f32 = 45.0;
const BLUE_COUNT: usize = 96;

const TRAIL_RADIUS: f32 = 18.0;

fn step(sled: &mut Sled, elapsed: f32) {
    let inner_color = Rgb::new(0.6, 0.93, 0.762);
    let outer_delta = Rgb::new(0.4, 0.51, 0.93);

    let inner_time_scale = elapsed / GREEN_RADIUS;
    let outer_time_scale = elapsed / BLUE_RADIUS;

    for i in 0..GREEN_COUNT {
        let angle = inner_time_scale + (TAU / GREEN_COUNT as f32) * i as f32;
        sled.get_at_angle_mut(angle).unwrap().color += inner_color;
    }

    for i in 0..BLUE_COUNT {
        let angle = outer_time_scale + (TAU / BLUE_COUNT as f32) * i as f32 % TAU;
        sled.get_at_angle_mut(angle).unwrap().color += outer_delta;
    }

    let radar_time_scale = elapsed / TRAIL_RADIUS;
    let angle = radar_time_scale % TAU;
    sled.map(|led| {
        let da = (led.angle() + angle) % TAU;
        let fac = 1.0 - (da / (TAU)).powf(1.25);
        led.color * fac
    });
}

fn trail(c: &mut Criterion) {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();

    let simulated_duration = 30.0;
    let simulated_hz = 144.0;
    let timestep = 1.0 / simulated_hz;
    let total_steps = (simulated_duration * simulated_hz) as usize;

    c.bench_function("quirky_trail", |b| {
        b.iter(|| {
            for i in 0..total_steps {
                step(&mut sled, timestep * i as f32);
            }
        })
    });
}

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