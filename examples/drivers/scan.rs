use palette::chromatic_adaptation::AdaptInto;
use rand::Rng;
use std::f32::consts::{PI, TAU};
use std::time::Duration;

use glam::Vec2;
use spatial_led::driver::{Driver, TimeInfo};
use spatial_led::driver_macros::*;
use spatial_led::BufferContainer;
use spatial_led::{color::Rgb, Sled, SledResult};

const SCAN_DURATION: f32 = 4.0;

#[allow(dead_code)]
pub fn build_driver() -> Driver {
    let mut driver = Driver::new();
    driver.set_startup_commands(startup);
    driver.set_compute_commands(compute);
    driver.set_draw_commands(draw);

    driver
}

fn rand_endpoints(sled: &Sled) -> (Vec2, Vec2) {
    let domain = sled.domain();
    let r = (domain.end - domain.start).length() * 0.6;
    let c = sled.center_point();

    let mut rng = rand::thread_rng();
    let start_angle = rng.gen_range(0.0..TAU);
    let end_angle = start_angle + PI;

    let start = c + (Vec2::from_angle(start_angle) * r);
    let end = c + (Vec2::from_angle(end_angle) * r);
    (start, end)
}

fn start_new_scan(sled: &Sled, buffers: &mut BufferContainer, now: Duration) {
    let t_buffer = buffers.create_buffer::<Duration>("times");

    t_buffer.push(now);
    t_buffer.push(now + Duration::from_secs_f32(SCAN_DURATION));

    let endpoints = buffers.create_buffer::<Vec2>("vectors");
    let (start, end) = rand_endpoints(&sled);
    endpoints.push(start); // v0 will be start point
    endpoints.push(end); // v1 will be end point
    endpoints.push(start); // v2 will be interpolation between v1 and v2
    endpoints.push((end - start).normalize()); // v3 will be direction of movement
}

#[startup_commands]
fn startup(sled: &mut Sled, buffers: &mut BufferContainer) -> SledResult {
    start_new_scan(sled, buffers, Duration::from_secs(0));
    Ok(())
}

#[compute_commands]
fn compute(sled: &Sled, buffers: &mut BufferContainer, time_info: &TimeInfo) -> SledResult {
    let t_buffer = buffers.get_buffer::<Duration>("times")?;
    let now = time_info.elapsed;
    let end_t = t_buffer[1];

    if now > end_t {
        start_new_scan(sled, buffers, time_info.elapsed);
        return Ok(());
    }

    let v_buffer = buffers.get_buffer_mut::<Vec2>("vectors")?;
    let start_p = v_buffer[0];
    let end_p = v_buffer[1];
    let a = 1.0 - ((end_t.as_secs_f32() - now.as_secs_f32()) / SCAN_DURATION);
    //println!("{}", a);
    v_buffer[2] = start_p.lerp(end_p, a);
    Ok(())
}

#[draw_commands]
fn draw(sled: &mut Sled, buffers: &BufferContainer, time_info: &TimeInfo) -> SledResult {
    // gradual fade to black
    let theta = ((time_info.elapsed.as_secs_f32() / 12.5).cos() + 1.0) * 180.0;
    sled.map(|led| led.color * (1.0 - time_info.delta.as_secs_f32() * 2.0));

    let v_buffer = buffers.get_buffer::<Vec2>("vectors")?;
    let scan_center = v_buffer[2];
    let scan_direction = v_buffer[3];

    // println!("{}", scan_center);
    let c: Rgb = spatial_led::color::oklch::Oklch::new(0.99, 0.3, theta).adapt_into();

    sled.set_at_dir_from(scan_direction.perp(), scan_center, c);
    sled.set_at_dir_from(-scan_direction.perp(), scan_center, c);

    Ok(())
}
