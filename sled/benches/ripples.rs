use rand::Rng;
use sled::driver::{BufferContainer, Driver, Filters, TimeInfo};
use sled::{color::Rgb, Sled, SledError, Vec2};
use std::{ops::Range, time::Duration};

const MAX_RIPPLES: usize = 12;
const MAX_RADIUS: f32 = 12.0;
const FEATHERING: f32 = 0.15;
const INV_F: f32 = 1.0 / FEATHERING;

fn ripples(c: &mut Criterion) {
    let sled = Sled::new("./benches/config1.toml").unwrap();
    let mut driver = Driver::new();
    driver.set_startup_commands(startup);
    driver.set_compute_commands(compute);
    driver.set_draw_commands(draw);
    driver.mount(sled);

    let simulated_duration = 30.0;
    let simulated_hz = 144.0;
    let total_steps = (simulated_duration * simulated_hz) as usize;
    let timestep = Duration::from_secs_f32(1.0 / simulated_hz);
    c.bench_function("ripples", |b| {
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
    targets = ripples
}
criterion_main!(benches);

fn startup(
    sled: &mut Sled,
    buffers: &mut BufferContainer,
    _filters: &mut Filters,
) -> Result<(), SledError> {
    let sled_bounds = sled.domain();

    let radii = buffers.create("radii");
    for _ in 0..MAX_RIPPLES {
        radii.push(rand_init_radius());
    }

    let positions = buffers.create("positions");
    for _ in 0..MAX_RIPPLES {
        positions.push(rand_point_in_range(&sled_bounds));
    }

    let colors = buffers.create("colors");
    colors.extend([
        Rgb::new(0.15, 0.5, 1.0),
        Rgb::new(0.25, 0.3, 1.0),
        Rgb::new(0.05, 0.4, 0.8),
        Rgb::new(0.7, 0.0, 0.6),
        Rgb::new(0.05, 0.75, 1.0),
        Rgb::new(0.1, 0.8, 0.6),
        Rgb::new(0.6, 0.05, 0.2),
        Rgb::new(0.85, 0.15, 0.3),
        Rgb::new(0.0, 0.0, 1.0),
        Rgb::new(1.0, 0.71, 0.705),
    ]);

    Ok(())
}

fn compute(
    sled: &Sled,
    buffers: &mut BufferContainer,
    _filters: &mut Filters,
    time_info: &TimeInfo,
) -> Result<(), SledError> {
    let delta = time_info.delta.as_secs_f32();
    let bounds = sled.domain();
    for i in 0..MAX_RIPPLES {
        let radius: f32 = buffers.get("radii", i).unwrap();
        if radius > MAX_RADIUS {
            let new_pos = rand_point_in_range(&bounds);
            let new_radius = rand_init_radius();
            buffers.set("positions", i, new_pos)?;
            buffers.set("radii", i, new_radius)?;
            continue;
        }

        let new_radius = radius + delta * inv_sqrt(radius.max(1.0));
        buffers.set("radii", i, new_radius)?;
    }
    Ok(())
}

fn rand_point_in_range(range: &Range<Vec2>) -> Vec2 {
    let mut rng = rand::thread_rng();
    Vec2::new(
        rng.gen_range(range.start.x * 1.25..range.end.x * 1.25),
        rng.gen_range(range.start.y * 1.25..range.end.y * 1.25),
    )
}

fn rand_init_radius() -> f32 {
    let mut rng = rand::thread_rng();
    // using a negative radius, we can scheudle a delay before the ripple actually appears
    rng.gen_range(-32.0..0.0)
}

fn draw(
    sled: &mut Sled,
    buffers: &BufferContainer,
    _filters: &Filters,
    _time_info: &TimeInfo,
) -> Result<(), SledError> {
    sled.set_all(Rgb::new(0.0, 0.0, 0.0));
    let colors = buffers.get_buffer("colors").unwrap();
    for i in 0..MAX_RIPPLES {
        let pos = buffers.get("positions", i).unwrap();
        let radius = buffers.get("radii", i).unwrap();

        if radius > -FEATHERING {
            draw_ripple_at(sled, pos, radius, colors[i % colors.len()]);
        }
    }

    // sled.map(|led| led.color / (Rgb::new(1.0, 1.0, 1.0) + led.color));
    Ok(())
}

fn draw_ripple_at(sled: &mut Sled, pos: Vec2, radius: f32, color: Rgb) {
    let inv_radius = 1.0 / radius;
    sled.modulate_within_dist_from(radius + FEATHERING, pos, |led| {
        let r = led.position().distance(pos);
        if r >= radius {
            let dist = r - radius;
            if dist < FEATHERING {
                let factor = (FEATHERING - dist) * INV_F;
                return led.color + color * (factor * inv_radius);
            }
        } else {
            let factor = r * inv_radius;
            return led.color + color * factor.powi(2) * inv_radius;
        }
        led.color
    });
}

fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}
