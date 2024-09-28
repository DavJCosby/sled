mod drivers;
use drivers::ripples;

mod resources;
use resources::tui::SledTerminalDisplay;

use spatial_led::{scheduler::Scheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/config.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Ripples", sled.domain());
    let mut driver = ripples::build_driver();
    driver.mount(sled);

    let mut scheduler = Scheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh()?;
        Ok(())
    });
}
