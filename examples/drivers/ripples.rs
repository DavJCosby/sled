use driver_macros::*;
use rand::Rng;
use sled::driver::{BufferContainer, Driver, TimeInfo};
use sled::SledResult;
use sled::{color::Rgb, Sled, Vec2};
use std::ops::Range;

const MAX_RIPPLES: usize = 12;
const MAX_RADIUS: f32 = 12.0;
const FEATHERING: f32 = 0.15;
const INV_F: f32 = 1.0 / FEATHERING;

#[allow(dead_code)]
pub fn build_driver() -> Driver {
    let mut driver = Driver::new();

    driver.set_startup_commands(startup);
    driver.set_compute_commands(compute);
    driver.set_draw_commands(draw);
    return driver;
}

#[startup_commands]
fn startup(sled: &mut Sled, buffers: &mut BufferContainer) -> SledResult {
    let sled_bounds = sled.domain();

    let radii = buffers.create_buffer("radii");
    for _ in 0..MAX_RIPPLES {
        radii.push(rand_init_radius());
    }

    let positions = buffers.create_buffer("positions");
    for _ in 0..MAX_RIPPLES {
        positions.push(rand_point_in_range(&sled_bounds));
    }

    let colors = buffers.create_buffer::<Rgb>("colors");
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

#[compute_commands]
fn compute(sled: &Sled, buffers: &mut BufferContainer, time_info: &TimeInfo) -> SledResult {
    let delta = time_info.delta.as_secs_f32();
    let bounds = sled.domain();
    for i in 0..MAX_RIPPLES {
        let radius: f32 = *buffers.get_buffer_item("radii", i)?;
        if radius > MAX_RADIUS {
            let new_pos = rand_point_in_range(&bounds);
            let new_radius = rand_init_radius();
            buffers.set_buffer_item("positions", i, new_pos)?;
            buffers.set_buffer_item("radii", i, new_radius)?;
            continue;
        }

        let new_radius = radius + delta * radius.max(1.0).sqrt().recip();
        buffers.set_buffer_item("radii", i, new_radius)?;
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

#[draw_commands]
fn draw(sled: &mut Sled, buffers: &BufferContainer) -> SledResult {
    sled.set_all(Rgb::new(0.0, 0.0, 0.0));
    let colors = buffers.get_buffer("colors")?;
    let positions = buffers.get_buffer("positions")?;
    let radii = buffers.get_buffer("radii")?;
    for i in 0..MAX_RIPPLES {
        let pos = positions[i];
        let radius = radii[i];

        if radius > -FEATHERING {
            draw_ripple_at(sled, pos, radius, colors[i % colors.len()]);
        }
    }

    // reinhard tonemapping
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
