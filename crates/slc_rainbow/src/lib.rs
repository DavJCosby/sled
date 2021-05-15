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
        let map = |angle: f32| {
            let dir = (angle.cos(), angle.sin());
            let lab = Lab {
                l: 50.0,
                a: (dir.0) * 100.0,
                b: (dir.1) * 100.0,
            };

            let rgb = lab.to_rgb();

            (rgb[0], rgb[1], rgb[2])
        };

        let mut write = controller.write().unwrap();

        let (range_min, range_max) = (-270.0 as f32, 80.0 as f32);

        write.map_angle_to_color_clamped(&map, range_min.to_radians(), range_max.to_radians());

        drop(write);
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
