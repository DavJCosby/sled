use sled::driver::Driver;
use sled::{color::Rgb, Sled, SledError};

// fn display

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./benches/config1.toml")?;
    let mut driver = Driver::new();

    driver.set_draw_commands(|sled, _time_info| {
        sled.set_all(Rgb::new(1.0, 1.0, 1.0));
        sled.set_at_angle(0.0, Rgb::new(1.0, 1.0, 1.0))?;
        Ok(())
    });

    driver.mount(sled);

    for _ in 0..10 {
        driver.update();
    }

    // sled.set_within_dist(1.0, Rgb::new(1.0, 0.0, 0.0));
    // sled.modulate_segment(2, |led| led.color * 0.5)?;

    // let _colors: Vec<Rgb<_, u8>> = sled.read_colors();

    Ok(())
}
