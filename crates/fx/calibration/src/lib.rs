use slc::prelude::*;

pub struct Calibration;

impl Calibration {
    pub fn new() -> Self {
        Calibration
    }
}

impl InputDevice for Calibration {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        let mut write = input_handle.write().unwrap();

        // set forward white
        write.set_at_view_dir((0.0, 1.0), (255, 255, 255), false);
        // set left red
        write.set_at_view_dir((-1.0, 0.0), (255, 0, 0), false);
        // set right green
        write.set_at_view_dir((1.0, 0.0), (0, 255, 0), false);
        // set backward blue
        write.set_at_view_dir((0.0, -1.0), (0, 0, 255), false);

        // find vertices
        let mut vertex_ids: Vec<usize> = vec![];
        let mut led_count = 0.0;
        for strip in write.room_data.strips() {
            led_count += strip.len() * write.room_data.density();
            vertex_ids.push(led_count as usize);
        }
        // set vertices yellow
        for id in vertex_ids {
            if id < write.room_data.leds().len() {
                write.set(id, (255, 150, 0));
            }
        }
    }

    fn stop(&mut self) {
        // one-shot application; do nothing
    }
}
