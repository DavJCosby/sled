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
        let mut write = controller.write().unwrap();

        for i in 0..3600 {
            let angle = (i as f32 / 10.0).to_radians();

            let dir = (angle.cos(), angle.sin());
            let lab = Lab {
                l: 70.0,
                a: (dir.0) * 100.0,
                b: (dir.1) * 100.0,
            };

            let rgb = lab.to_rgb();
            write.set_at_room_dir(dir, (rgb[0], rgb[1], rgb[2]));
        }
        
        drop(write);
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
