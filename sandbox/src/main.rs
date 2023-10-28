use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;

    sled.for_each_in_segment(0, |led, alpha| {
        *led = Rgb::new(alpha, 1.0, 1.0);
    })?;

    sled.for_each_in_segment(1, |led, alpha| {
        *led = Rgb::new(1.0, alpha, 1.0);
    })?;

    sled.for_each_in_segment(2, |led, alpha| {
        *led = Rgb::new(1.0, 1.0, alpha);
    })?;

    sled.for_each_in_segment(3, |led, alpha| {
        *led = Rgb::new(alpha, 0.0, 1.0 - alpha);
    })?;

    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }
    println!("");

    Ok(())
}
