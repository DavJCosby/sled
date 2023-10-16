use colored::Colorize;
use palette::Srgb;
use sled::{Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;
    sled.set_all(Srgb::new(1.0, 0.0, 0.0));

    let new_colors = sled.get_colors();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.0, color.1, color.2));
    }

    Ok(())
}
