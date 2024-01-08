mod drivers;
use drivers::ripples;

mod resources;
use resources::tui::SledTerminalDisplay;

use sled::{scheduler::Scheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Ripples", sled.domain());
    let mut driver = ripples::build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::fixed_hz(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.leds = driver.read_colors_and_positions();
        display.refresh()?;
        Ok(())
    });
}
