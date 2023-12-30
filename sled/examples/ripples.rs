mod tui;

use std::ops::Range;
use std::time::Instant;

use rand::Rng;
use tui::SledTerminalDisplay;

use sled::driver::{Driver, Filters, Sliders, TimeInfo};
use sled::{color::Rgb, scheduler::Scheduler, Sled, SledError, Vec2};

const MAX_RIPPLES: usize = 10;
const MAX_RADIUS: f32 = 12.0;

const FEATHERING: f32 = 0.15;
const INV_F: f32 = 1.0 / FEATHERING;

// I'm dumb and couldn't find a performant way to turn a usize into a &str
const STR_IDS: [&str; MAX_RIPPLES] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const COLS: [Rgb; MAX_RIPPLES] = [
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
];

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
    rng.gen_range(-25.0..0.0)
}

fn draw(
    sled: &mut Sled,
    sliders: &Sliders,
    _filters: &Filters,
    _time_info: &TimeInfo,
) -> Result<(), SledError> {
    sled.set_all(Rgb::new(0.0, 0.0, 0.0));
    for i in 0..MAX_RIPPLES {
        try_draw_ripple(sled, sliders, i);
    }

    //sled.map(|led| led.color / (Rgb::new(1.0, 1.0, 1.0) + led.color));

    Ok(())
}

fn try_draw_ripple(sled: &mut Sled, sliders: &Sliders, id: usize) {
    let str_id = STR_IDS[id];
    let pos = sliders.get::<Vec2>(str_id).unwrap();
    let radius = sliders.get::<f32>(str_id).unwrap();

    if radius > -FEATHERING {
        draw_ripple_at(sled, pos, radius, COLS[id]);
    }
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

fn main() {
    let sled = Sled::new("./examples/config.toml").unwrap();
    let sled_bounds = sled.domain();
    let mut display = SledTerminalDisplay::start("Sled Visualizer", sled.domain());

    let mut driver = Driver::new();

    driver.set_draw_commands(draw);
    driver.mount(sled);

    let mut scheduler = Scheduler::fixed_hz(500.0);
    let mut last_update = Instant::now();

    let mut positions: [Vec2; MAX_RIPPLES] = [Vec2::new(0.0, 0.0); MAX_RIPPLES];
    let mut radii: [f32; MAX_RIPPLES] = [0.0; MAX_RIPPLES];

    for i in 0..MAX_RIPPLES {
        positions[i] = rand_point_in_range(&sled_bounds);
        radii[i] = rand_init_radius();
        driver.set_slider::<Vec2>(STR_IDS[i], positions[i]);
    }

    scheduler.loop_until_err(|| {
        let delta = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        for i in 0..MAX_RIPPLES {
            let str_id = STR_IDS[i];
            if radii[i] > MAX_RADIUS {
                positions[i] = rand_point_in_range(&sled_bounds);
                radii[i] = rand_init_radius();
                driver.set_slider::<Vec2>(str_id, positions[i]);
            }

            radii[i] += delta * inv_sqrt(radii[i].max(1.0));
            driver.set_slider::<f32>(str_id, radii[i]);
        }

        driver.step();
        display.set_title(format!("{} FPS", (1.0 / delta).ceil() as usize));
        display.leds = driver.read_colors_and_positions();
        display.refresh()?;
        Ok(())
    });
}
