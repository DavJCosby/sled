mod drivers;
use drivers::scan;

mod resources;
use resources::tui::SledTerminalDisplay;

use sled::{scheduler::Scheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/config.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Scan", sled.domain());
    let mut driver = scan::build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh()?;
        Ok(())
    });
}