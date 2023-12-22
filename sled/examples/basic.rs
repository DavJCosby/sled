use sled::driver::Driver;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./benches/config1.toml")?;
    let mut driver = Driver::new();

    driver.set_draw_commands(|sled, _time_info| {
        sled.set_all(Rgb::new(1.0, 1.0, 1.0));
        sled.set_at_angle(0.0, Rgb::new(1.0, 1.0, 1.0))?;
        Ok(())
    });

    let num: &f32 = driver.get_slider("hello").unwrap();

    // let mut sliders = Sliders::new();
    // sliders.set("background", Rgb::new(1.0, 1.0, 1.0));
    // sliders.set("brightness", 0.5);

    // let bg: &Rgb = sliders.get("background").unwrap();
    // let brightness: &f32 = sliders.get("brightness").unwrap();

    Ok(())
}
