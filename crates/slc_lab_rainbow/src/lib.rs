use std::{f32::consts::PI, thread, time::Instant};

use lab::Lab;
use slc::prelude::*;
pub struct Rainbow {
    stop: bool,
}

impl Rainbow {
    pub fn new() -> Rainbow {
        Rainbow { stop: false }
    }
}

impl InputDevice for Rainbow {
    fn start(self, controller: std::sync::Arc<std::sync::RwLock<RoomController>>) {
        thread::spawn(move || {
            let start = Instant::now();

            let mut last = 0.0;
            while !self.stop {
                let duration = start.elapsed().as_secs_f32() / 2.0;
                if duration - last < 0.008333 {
                    //thread::sleep(Duration::from_millis(1));
                    continue;
                };

                let color_map = |r: f32| {
                    let (dy, dx) = (r * 2.0 + duration * 0.0).sin_cos();
                    let lab = Lab {
                        l: 36.67,
                        a: dx * 100.0,
                        b: dy * 100.0,
                    };

                    let rgb = lab.to_rgb();

                    (rgb[0], rgb[1], rgb[2])
                };

                let mut write = controller.write().unwrap();
                write.set_all((0, 0, 0));
                write.map_angle_to_color_clamped(
                    &color_map,
                    duration % (2.0 * PI),
                    (duration + 1.0) % (2.0 * PI),
                );

                drop(write);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
