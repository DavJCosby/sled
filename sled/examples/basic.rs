use sled::{
    color::{Rgb, Srgb},
    Sled, SledError,
};

// fn display

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();

    sled.set_within_dist(1.0, Rgb::new(1.0, 0.0, 0.0));

    sled.modulate_segment(2, |led| led.color * 0.5);

    let led = sled.get_at_dir(glam::Vec2::new(0.0, -1.0)).unwrap();
    println!("{}", led.angle());

    let colors: Vec<Srgb<u8>> = sled.read_colors();

    Ok(())
}
