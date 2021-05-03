use core::time;
use std::thread;
use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use slc::core::{room::Room, room_controller::RoomController};
use slc::gui::Gui;
use slc::output::OutputDevice;

pub fn main() {
    let room = Room::new_from_file("room_configs/config1.rcfg");
    let room_controller = RoomController { room };

    let rw_lock = Arc::new(RwLock::new(room_controller));
    let write_clone = rw_lock.clone();

    thread::spawn(move || {
        println!("entered");
        let start = Instant::now();
        loop {
            let duration = start.elapsed().as_secs_f32();
            let x = duration.cos();
            let y = duration.sin();
            let mut controller_write = write_clone.write().unwrap();
            controller_write.set_led_at_dir(
                (x, y),
                (
                    (((duration / 3.0).sin() * 255.0).abs() as u8),
                    100,
                    ((duration / 3.0).cos() * 255.0).abs() as u8,
                ),
            );
            drop(controller_write);
            //thread::sleep(time::Duration::from_millis(10));
        }
    });

    Gui::start(rw_lock);
}
