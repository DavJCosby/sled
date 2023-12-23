use sled::driver::Driver;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./benches/config1.toml")?;
    let mut driver = Driver::new();

    driver.set_startup_commands(|sled, sliders, filters| {
        sliders.set("background", Rgb::new(0.0, 0.0, 0.0));
        sliders.set("light_color", Rgb::new(1.0, 1.0, 1.0));

        filters.set("left_wall", sled.get_segment(2).unwrap());
        filters.set("cone", sled.filter_by_angle(|a| a > 0.2 && a <= 0.6));
        Ok(())
    });

    driver.set_draw_commands(|sled, sliders, filters, time_info| {
        let bg_color = sliders.get("background").unwrap_or_default();
        let light_color = sliders.get("light_color").unwrap_or_default();

        let cone = filters.get("cone").unwrap();
        let left_wall = filters.get("left_wall").unwrap();

        let peak_br = (time_info.elapsed.as_secs_f32() / 20.0).sin() + 1.0;

        sled.set_all(bg_color);
        sled.set_filter(cone, light_color);
        sled.modulate_filter(left_wall, |led| {
            let d_sq = led.distance().powi(2);
            light_color * peak_br / d_sq
        });

        Ok(())
    });

    driver.mount(sled);

    for _ in 0..10 {
        driver.update();
    }

    let sled = driver.dismount();

    Ok(())
}
