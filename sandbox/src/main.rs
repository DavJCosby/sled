use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;

    let max = sled.num_leds();

    sled.for_each(|led, index| {
        *led = Rgb::new(index as f32 / max as f32, 0.5, 0.5);
    });

    // sled.for_each_in_segment(1, |led, alpha| {
    //     *led += Rgb::new(0.0, alpha, 0.0);
    // })?;

    // sled.for_each_in_segment(2, |led, alpha| {
    //     *led += Rgb::new(0.0, 1.0 - alpha, 0.0);
    // })?;

    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }
    println!("");

    Ok(())
}
