use std::sync::Arc;
use std::{thread, time::Instant};

use slc::prelude::*;

const UPDATE_TIMING: f32 = 1.0 / 240.0;

pub struct Canvas {
    stop: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            stop: false,
        }
    }
}

impl InputDevice for Canvas {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        let stop = Arc::new(self.stop);

        thread::spawn(move || {
            let start = Instant::now();
            let mut last = 0.0;

            while !*stop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                };

                let mut controller_write = input_handle.write().unwrap();

                controller_write.set_all((5, 4, 0));

                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
