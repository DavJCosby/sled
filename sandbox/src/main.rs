use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;
    sled.set_all(Rgb::new(1.0, 0.0, 0.0));
    sled.set_range(2..100, Rgb::new(0.0, 0.5, 0.5))?;

    let range = sled.get_range_mut(200..250);
    for color in range {
        *color = Rgb::new(0.0, 0.0, 0.0);
    }

    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }

    Ok(())
}
