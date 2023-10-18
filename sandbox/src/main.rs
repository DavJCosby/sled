use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;
    sled.set_all(Rgb::new(1.0, 1.0, 1.0));

    for led in sled.get_vertices_mut() {
        *led += Rgb::new(0.0, 0.0, 1.0);
    }

    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }
    println!("");

    Ok(())
}
