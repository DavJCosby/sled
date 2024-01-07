mod tui;

use glam::Vec2;
use tui::SledTerminalDisplay;
use sled::{color::Rgb, Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./examples/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Sled Visualizer", sled.domain());
    
    sled.set_vertices(Rgb::new(0.75, 0.75, 0.75));
    sled.set_at_dir(Vec2::new(1.0, 0.0), Rgb::new(1.0, 0.0, 0.0))?;
    sled.set_at_dir(Vec2::new(-1.0, 0.0), Rgb::new(0.5, 0.0, 0.0))?;
    sled.set_at_dir(Vec2::new(0.0, 1.0), Rgb::new(0.0, 1.0, 0.0))?;
    sled.set_at_dir(Vec2::new(0.0, -1.0), Rgb::new(0.0, 0.5, 0.0))?;

    display.leds = sled.read_colors_and_positions();
    display.refresh().unwrap();
    Ok(())
}
