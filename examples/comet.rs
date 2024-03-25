mod drivers;
use drivers::comet;

mod resources;
use resources::tui::SledTerminalDisplay;

use sled::{scheduler::Scheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Comet", sled.domain());
    let mut driver = comet::build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.leds = driver.colors_and_positions();
        display.refresh()?;
        Ok(())
    });
}
