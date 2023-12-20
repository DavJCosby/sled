use sled::{color::Rgb, Sled, SledError};

// fn display

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./benches/config1.toml")?;

    sled.set_within_dist(1.0, Rgb::new(1.0, 0.0, 0.0));
    sled.modulate_segment(2, |led| led.color * 0.5)?;

    let _colors: Vec<Rgb<_, u8>> = sled.read_colors();

    Ok(())
}
