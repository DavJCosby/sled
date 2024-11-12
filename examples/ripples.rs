mod drivers;
use drivers::ripples;

mod resources;
use resources::tui::SledTerminalDisplay;

use spatial_led::{scheduler::StdScheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Ripples", sled.domain());
    let mut driver = ripples::build_driver();
    driver.mount(sled);

    let mut scheduler = StdScheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh()?;
        Ok(())
    });
}
