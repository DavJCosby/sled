mod resources;
use glam::Vec2;
use palette::rgb::Srgb;
use resources::tui::SledTerminalDisplay;
use spatial_led::{Sled, SledError};

fn main() -> Result<(), SledError> {
    let mut sled = Sled::new("./examples/resources/complex_room.yap")?;

    let mut display = SledTerminalDisplay::start("Calibration", sled.domain());

    sled.set_all(Srgb::new(0.1, 0.1, 0.1));
    sled.set_vertices(Srgb::new(0.75, 0.75, 0.75));
    sled.set_at_dir(Vec2::new(1.0, 0.0), Srgb::new(1.0, 0.0, 0.0));
    sled.set_at_dir(Vec2::new(-1.0, 0.0), Srgb::new(0.5, 0.0, 0.0));
    sled.set_at_dir(Vec2::new(0.0, 1.0), Srgb::new(0.0, 1.0, 0.0));
    sled.set_at_dir(Vec2::new(0.0, -1.0), Srgb::new(0.0, 0.5, 0.0));

    display.set_leds(sled.colors_and_positions());
    display.refresh().unwrap();
    Ok(())
}
