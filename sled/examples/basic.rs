use sled::{Rgb, Sled, SledError};


// fn display

fn main() {
    let mut sled = Sled::new("./benches/config1.toml").unwrap();
    let white = Rgb::new(1.0, 1.0, 1.0);

    sled.set_range(20..50, white);
}