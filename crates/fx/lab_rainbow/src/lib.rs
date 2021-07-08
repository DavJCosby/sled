use lab::Lab;
use slc::prelude::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Instant,
};

const UPDATE_TIMING: f32 = 1.0 / 144.0;

pub struct Rainbow {
    alive: Arc<AtomicBool>,
    spin_speed: f32,
    scale: f32,
}

impl Rainbow {
    pub fn new(spin_speed: f32, scale: f32) -> Rainbow {
        Rainbow {
            alive: Arc::new(AtomicBool::new(false)),
            spin_speed,
            scale,
        }
    }
}

impl InputDevice for Rainbow {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        self.alive.store(true, Ordering::SeqCst);
        let alive = self.alive.clone();

        let scale = self.scale;
        let spin_speed = self.spin_speed;
        thread::spawn(move || {
            let start = Instant::now();
            let mut last = 0.0;

            while alive.load(Ordering::SeqCst) {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                };

                let color_map = |r: f32| {
                    let (dy, dx) = (r * scale + duration * spin_speed).sin_cos();
                    let lab = Lab {
                        l: 36.67,
                        a: dx * 100.0,
                        b: dy * 100.0,
                    };

                    let rgb = lab.to_rgb();

                    (rgb[0], rgb[1], rgb[2])
                };

                let mut write = input_handle.write().unwrap();
                write.map_angle_to_color(&color_map);

                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
