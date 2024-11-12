mod drivers;
use drivers::comet;

mod resources;
use resources::tui::SledTerminalDisplay;

use spatial_led::{scheduler::StdScheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/complex_room.yap").unwrap();
    let mut display = SledTerminalDisplay::start("Comet", sled.domain());
    let mut driver = comet::build_driver();
    driver.mount(sled);

    let mut scheduler = StdScheduler::new(500.0);
    scheduler.loop_until_err(|| {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh()?;
        Ok(())
    });
}
