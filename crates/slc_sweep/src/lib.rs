use std::{sync::RwLock, thread, time::Instant};

use slc::room_controller::RoomController;
use slc::devices::InputDriver;

pub struct Sweep {
    stop: bool,
}

impl Sweep {
    pub fn new() -> Sweep {
        Sweep { stop: false }
    }
}

impl InputDriver for Sweep {
    fn start(self, controller_copy: std::sync::Arc<RwLock<RoomController>>) {
        thread::spawn(move || {
            let start = Instant::now();
            while !self.stop == true {
                let duration = start.elapsed().as_secs_f32();
                let x = duration.cos();
                let y = duration.sin();
                let mut controller_write = controller_copy.write().unwrap();
                controller_write.set_led_at_dir(
                    (x, y),
                    (
                        (((duration / 3.0).sin() * 255.0).abs() as u8),
                        50,
                        ((duration / 3.0).cos() * 255.0).abs() as u8,
                    ),
                );
                drop(controller_write);
                //thread::sleep(time::Duration::from_millis(10));
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
