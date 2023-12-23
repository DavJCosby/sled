use sled::driver::Driver;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./benches/config1.toml")?;
    let mut driver = Driver::new();

    driver.set_startup_commands(|_sled, sliders| {
        println!("Startup");
        sliders.set("brightness", 0.5);
        sliders.set("color", Rgb::new(1.0, 1.0, 1.0));
        Ok(())
    });

    driver.set_draw_commands(|sled, sliders, _time_info| {
        let brightness: f32 = *sliders.get("brightness").unwrap();
        let color: Rgb = *sliders.get("color").unwrap();

        println!("{}", brightness);

        sled.set_all(color * brightness);
        Ok(())
    });

    driver.mount(sled);

    for _ in 0..10 {
        driver.update();
    }

    Ok(())
}
