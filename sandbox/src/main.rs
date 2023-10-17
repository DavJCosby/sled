use colored::Colorize;
use sled::{color::Srgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;
    sled.set_all(Srgb::new(1.0, 0.0, 0.0));

    let new_colors = sled.get_colors();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }

    Ok(())
}
