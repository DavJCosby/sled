use std::f32::consts::PI;

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
        let color_map = |(dx, dy): (f32, f32)| {
            let lab = Lab {
                l: 65.0,
                a: dx * 100.0,
                b: dy * 100.0,
            };

            let rgb = lab.to_rgb();

            (rgb[0], rgb[1], rgb[2])
        };

        let mut write = controller.write().unwrap();

        write.map_dir_to_color(&color_map);

        drop(write);
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
