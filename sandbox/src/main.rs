use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;
    //sled.set_all(Rgb::new(1.0, 0.0, 0.0));
    // sled.set_range(2..150, Rgb::new(0.0, 0.5, 0.5))?;

    // for color in sled.get_range_mut(200..250) {
    //     *color = Rgb::new(0.0, 0.0, 0.0);
    // }

    // for color in sled.get_range_mut(100..200) {
    //     *color /= 2.0;
    // }

    sled.set_segment(0, Rgb::new(1.0, 1.0, 1.0))?;
    sled.set_segment(1, Rgb::new(1.0, 0.0, 0.0))?;
    sled.set_segment(2, Rgb::new(0.0, 1.0, 0.0))?;
    sled.set_segment(3, Rgb::new(0.0, 0.0, 1.0))?;


    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }
    println!("");

    Ok(())
}
