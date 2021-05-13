use std::{f32::consts::PI, sync::RwLock, thread, time::Instant};

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
            while !self.stop == true {
                let duration = start.elapsed().as_secs_f32();
                let x = duration.cos();
                let y = duration.sin();
                let mut controller_write = controller_copy.write().unwrap();

                // set all pixels white
                controller_write.set_all((255, 255, 255));
                // set the LED leftmost to the camera red
                controller_write.set_at_view_dir((-0.0, -1.0), (255, 0, 0));
                // set the LED rightmost to the camera green
                controller_write.set_at_view_angle(PI, (0, 255, 0));
                // set the northmost LED blue (relative to the room's coordinate space)
                controller_write.set_at_room_dir((0.0, 1.0), (0, 0, 255));
                // controller_write.set_at_room_dir(
                //     (x, y),
                //     (
                //         (((duration / 3.0).sin() * 255.0).abs() as u8),
                //         50,
                //         ((duration / 3.0).cos() * 255.0).abs() as u8,
                //     ),
                // );
                drop(controller_write);
                //thread::sleep(time::Duration::from_millis(10));
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
