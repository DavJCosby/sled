use std::{
    sync::RwLock,
    thread,
    time::{Duration, Instant},
};

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

            while !self.stop {
                let duration = start.elapsed().as_secs_f32() / 2.0;
                if duration - last < 0.008333 {
                    //thread::sleep(Duration::from_millis(1));
                    continue;
                };
                let x = duration.cos();
                let y = duration.sin();

                let x2 = (duration + 0.005).cos();
                let y2 = (duration + 0.005).sin();

                let x3 = (duration - 0.005).cos();
                let y3 = (duration - 0.005).sin();

                let mut controller_write = controller_copy.write().unwrap();

                controller_write.set_all((0, 0, 0));
                controller_write.set_at_room_dir((x, y), (0, 255, 0));
                controller_write.set_at_room_dir((x2, y2), (255, 0, 0));
                controller_write.set_at_room_dir((x3, y3), (0, 0, 255));

                drop(controller_write);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
