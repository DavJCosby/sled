use lab::Lab;
use slc::prelude::*;
use std::{thread, time::Instant};

const UPDATE_TIMING: f32 = 1.0 / 240.0;

pub struct Rainbow {
    stop: bool,
    spin_speed: f32,
    scale: f32,
}

impl Rainbow {
    pub fn new(spin_speed: f32, scale: f32) -> Rainbow {
        Rainbow {
            stop: false,
            spin_speed,
            scale,
        }
    }
}

impl InputDevice for Rainbow {
    fn start(self, controller: std::sync::Arc<std::sync::RwLock<RoomController>>) {
        thread::spawn(move || {
            let start = Instant::now();
            let mut last = 0.0;

            while !self.stop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                };

                let color_map = |r: f32| {
                    let (dy, dx) = (r * self.scale + duration * self.spin_speed).sin_cos();
                    let lab = Lab {
                        l: 36.67,
                        a: dx * 100.0,
                        b: dy * 100.0,
                    };

                    let rgb = lab.to_rgb();

                    (rgb[0], rgb[1], rgb[2])
                };

                let mut write = controller.write().unwrap();
                write.map_angle_to_color(&color_map);

                drop(write);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
