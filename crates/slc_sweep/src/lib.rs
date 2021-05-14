use core::time;
use std::{f32::consts::PI, sync::RwLock, thread, time::Instant};

use slc::devices::InputDevice;
use slc::room_controller::RoomController;

pub struct Sweep {
    stop: bool,
}

impl Sweep {
    pub fn new() -> Sweep {
        Sweep { stop: false }
    }
}

impl InputDevice for Sweep {
    fn start(self, controller_copy: std::sync::Arc<RwLock<RoomController>>) {
        thread::spawn(move || {
            let start = Instant::now();

            let mut last = 0.0;
            while !self.stop == true {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < 0.0025 { continue };
                let x = duration.cos();
                let y = duration.sin();
                let mut controller_write = controller_copy.write().unwrap();

                for led in &mut controller_write.room.leds {
                    *led = (
                        (led.0 as f32 * 0.999) as u8,
                        (led.1 as f32 * 0.999) as u8,
                        (led.2 as f32 * 0.999) as u8
                    );
                }

                controller_write.set_at_room_dir((x, y), (0, 255, 0));
                drop(controller_write);
                last = duration;
                //thread::sleep(time::Duration::from_millis(1));
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
