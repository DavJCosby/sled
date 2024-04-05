mod drivers;
use drivers::warpspeed;

mod resources;
use resources::tui::SledTerminalDisplay;

use sled::{scheduler::Scheduler, Sled};

fn main() {
    let sled = Sled::new("./examples/resources/config.toml").unwrap();
    let mut display = SledTerminalDisplay::start("Warpspeed", sled.domain());
    let mut driver = warpspeed::build_driver();
    driver.mount(sled);

    //let mut scheduler = Scheduler::new(500.0);
    //scheduler.loop_until_err(|| {
    loop {
        driver.step();
        display.set_leds(driver.colors_and_positions_coerced());
        display.refresh().unwrap();
        //Ok(())
    }
    //});
}
