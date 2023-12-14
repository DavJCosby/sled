use sled::{color::Rgb, Sled, SledError};

// fn display

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();

    let led = sled.get_at_dir(glam::Vec2::new(0.0, -1.0)).unwrap();
    println!("{}", led.angle());
    Ok(())
}
