use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Instant};

use slc::prelude::*;

const UPDATE_TIMING: f32 = 1.0 / 240.0;

pub struct Sweep {
    alive: Arc<AtomicBool>,
    spin_speed: f32,
}

impl Sweep {
    pub fn new(spin_speed: f32) -> Sweep {
        Sweep {
            alive: Arc::new(AtomicBool::new(false)),
            spin_speed,
        }
    }
}

impl InputDevice for Sweep {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();
        let spin_speed = self.spin_speed;

        thread::spawn(move || {
            let start = Instant::now();
            let mut last = 0.0;

            while alive.load(Ordering::SeqCst) {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                };

                let mut controller_write = input_handle.write().unwrap();

                controller_write.set_all((0, 0, 0));
                let (y, x) = (duration * spin_speed).sin_cos();
                controller_write.set_at_room_dir((x, y), (0, 255, 0), true);

                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
