use std::{
    sync::{Arc, RwLock},
    thread,
};

use crate::prelude::*;

pub struct Room<'a> {
    room_controller_handle: Arc<RwLock<RoomController>>,
    input_device: Option<Box<dyn InputDevice + 'a>>,
    output_devices: Vec<Box<dyn OutputDevice + 'a>>,
    running: bool,
}

impl<'a> Room<'a> {
    pub fn new(filepath: &str) -> Self {
        let rc = RoomController::new(filepath);
        let rc_handle = Arc::new(RwLock::new(rc));

        Room {
            room_controller_handle: rc_handle,
            input_device: None,
            output_devices: vec![],
            running: false,
        }
    }

    pub fn set_input(&mut self, input: impl InputDevice + 'static) {
        self.input_device = Some(Box::new(input));
    }

    pub fn add_output<O: OutputDevice + 'a>(&mut self, output: O) {
        self.output_devices.push(Box::new(output));
    }

    pub fn start(&mut self) {
        if let Some(input_device) = &self.input_device {
            input_device.start(self.room_controller_handle.clone());
        } else {
            eprintln!("No input device.");
        }

        for device in &self.output_devices {
            device.start(self.room_controller_handle.clone());
        }
    }

    pub fn stop(&mut self) {
        if let Some(input_device) = &mut self.input_device {
            input_device.stop();
        } else {
            eprintln!("No input device.");
        }
    }
}
