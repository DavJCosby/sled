use sled::{color::Rgb, Sled, SledError, Vec2};

// fn display

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./benches/config1.toml")?;

    sled.set_within_dist(1.0, Rgb::new(1.0, 0.0, 0.0));

    sled.modulate_segment(2, |led| led.color * 0.5)?;

    let led = sled.get_at_dir(Vec2::new(0.0, -1.0)).unwrap();
    println!("{}", led.angle());

    let _colors: Vec<Rgb<_, u8>> = sled.read_colors();

    Ok(())
}
