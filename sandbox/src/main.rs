use colored::Colorize;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./cfg/config1.toml")?;

    for i in 0..sled.num_vertices() {
        let led = 
        *sled.get_vertex_mut(i).unwrap() = Rgb::new(i as f32 * 0.25, 1.0, 1.0);
    }

    let new_colors = sled.read();
    for color in new_colors {
        print!("{}", "⬤ ".truecolor(color.red, color.green, color.blue));
    }
    println!("");

    Ok(())
}
