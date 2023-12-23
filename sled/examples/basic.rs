use sled::driver::Driver;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./benches/config1.toml")?;
    let mut driver = Driver::new();

    driver.set_startup_commands(|sled, sliders, sets| {
        sliders.set("background", Rgb::new(0.0, 0.0, 0.0));
        sliders.set("light_color", Rgb::new(1.0, 1.0, 1.0));

        sets.set("left_wall", sled.get_segment(2).unwrap());
        sets.set("cone", sled.filter_by_angle(|a| a > 0.2 && a <= 0.6));
        Ok(())
    });

    driver.set_draw_commands(|sled, sliders, sets, time_info| {
        let bg_color: Rgb = *sliders.get("background").unwrap();
        let light_color: Rgb = *sliders.get("light_color").unwrap();

        let cone = sets.get("cone").unwrap();
        let left_wall = sets.get("left_wall").unwrap();

        let peak_br = (time_info.elapsed.as_secs_f32() / 20.0).sin() + 1.0;

        sled.set_all(bg_color);

        sled.set_leds_in_set(cone, light_color);

        sled.modulate_leds_in_set(left_wall, |led| {
            let d_sq = led.distance().powi(2);
            light_color * peak_br / d_sq
        });

        Ok(())
    });

    driver.mount(sled);

    for _ in 0..10 {
        driver.update();
    }

    Ok(())
}
