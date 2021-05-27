use slc::prelude::*;

pub struct Calibration;

impl Calibration {
    pub fn new() -> Self {
        Calibration
    }
}

impl InputDevice for Calibration {
    fn start(self, controller: std::sync::Arc<std::sync::RwLock<RoomController>>) {
        let mut write = controller.write().unwrap();

        // set forward white
        write.set_at_view_dir((0.0, 1.0), (255, 255, 255));
        // set left red
        write.set_at_view_dir((-1.0, 0.0), (255, 0, 0));
        // set right green
        write.set_at_view_dir((1.0, 0.0), (0, 255, 0));
        // set backward blue
        write.set_at_view_dir((0.0, -1.0), (0, 0, 255));

        // find vertices
        let mut vertex_ids: Vec<usize> = vec![];
        let mut led_count = 0.0;
        for strip in write.room.strips() {
            led_count += strip.len() * write.room.density();
            vertex_ids.push(led_count as usize);
        }
        // set vertices yellow
        for id in vertex_ids {
            if id < write.room.leds().len() {
                write.set(id, (255, 255, 0));
            }
        }
    }

    fn stop(&mut self) {
        // one-shot application; do nothing
    }
}
