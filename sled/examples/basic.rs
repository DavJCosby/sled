use sled::driver::{Driver, Filters, Scheduler, Sliders, TimeInfo};
use sled::{color::Rgb, Sled, SledError};

fn startup(sled: &mut Sled, sliders: &mut Sliders, filters: &mut Filters) -> Result<(), SledError> {
    sliders.set("background", Rgb::new(0.0, 0.0, 0.0));
    sliders.set("light_color", Rgb::new(1.0, 1.0, 1.0));

    filters.set("left_wall", sled.get_segment(2).unwrap());
    filters.set("cone", sled.filter_by_angle(|a| a > 0.2 && a <= 0.6));
    Ok(())
}

fn draw(
    sled: &mut Sled,
    sliders: &Sliders,
    filters: &Filters,
    time_info: &TimeInfo,
) -> Result<(), SledError> {
    let bg_color = sliders.get("background").ok_or("bg_color not found")?;
    let light_color = sliders.get("light_color").ok_or("light_color not found")?;

    let cone = filters.get("cone").ok_or("cone not found")?;
    let left_wall = filters.get("left_wall").ok_or("left_wall not found")?;

    let peak_br = (time_info.elapsed.as_secs_f32() / 20.0).sin() + 1.0;

    sled.set_all(bg_color);
    sled.set_filter(cone, light_color);
    sled.modulate_filter(left_wall, |led| {
        let d_sq = led.distance().powi(2);
        light_color * peak_br / d_sq
    });

    Ok(())
}

fn main() -> Result<(), SledError> {
    let mut driver = Driver::new();
    driver.set_startup_commands(startup);
    driver.set_draw_commands(draw);

    let sled = Sled::new("./benches/config1.toml")?;
    driver.mount(sled);

    let mut scheduler = Scheduler::fixed_hz(120.0);
    scheduler.loop_forever(|| {
        driver.update();
    });
}
