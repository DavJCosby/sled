use std::{sync::RwLock, thread, time::Instant};

use slc::room_controller::RoomController;
use slc::{devices::InputDevice, prelude::LineSegmentTrait};

const UPDATE_TIMING: f32 = 1.0 / 240.0;

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

            let mut controller_write = controller_copy.write().unwrap();

            println!("len: {}", controller_write.room.leds().len());

            controller_write.set_at_view_dir((0.0, 1.0), (0, 255, 0));
            controller_write.set_at_view_dir((-1.0, 0.0), (255, 0, 0));
            controller_write.set_at_view_dir((1.0, 0.0), (255, 0, 0));

            // controller_write.set(37, (255, 255, 255));
            // controller_write.set(212, (255, 255, 255));
            // controller_write.set(277, (255, 255, 255));
            // controller_write.set(333, (255, 255, 255));
            // controller_write.set(333, (255, 255, 255));
            // controller_write.set(470, (255, 255, 255));
            // controller_write.set(653, (255, 255, 255));

            let mut tot = 0.0;
            let mut a: Vec<usize> = vec![];
            for strip in controller_write.room.strips() {
                tot += strip.len() * controller_write.room.density();
                a.push(tot as usize);
                //println!("{}", strip.len() * controller_write.room.density());
            }

            for id in a {
                if id < 660 {
                    controller_write.set(id, (255, 255, 255));
                } else {
                    //println!("oop {}", id);
                }
            }

            drop(controller_write);

            while !self.stop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                };

                let mut controller_write = controller_copy.write().unwrap();
                controller_write.set_all((0, 0, 0));

                let num = 8;

                for i in 0..num {
                    let x = (duration / 3.0 + (i as f32 / num as f32) * 6.2831853).cos();
                    let y = (duration / 3.0 + (i as f32 / num as f32) * 6.2831853).sin();
                    controller_write.set_at_room_dir((x, y), (0, 255, 0));
                }

                drop(controller_write);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
