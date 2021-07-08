use std::sync::{Arc, RwLock};

use crate::prelude::*;

pub struct Room<'a> {
    rc_lock: Arc<RwLock<RoomController>>,
    input_device: Option<Box<dyn InputDevice + 'a + Send + Sync>>,
    output_devices: Vec<Box<dyn OutputDevice + 'a + Send + Sync>>,
    running: bool,
}

impl<'a> Room<'a> {
    pub fn new(filepath: &str) -> Self {
        let rc = RoomController::new(filepath);
        let rc_lock = Arc::new(RwLock::new(rc));

        Room {
            rc_lock,
            input_device: None,
            output_devices: vec![],
            running: false,
        }
    }

    pub fn set_input_device<I: InputDevice + 'a + Send + Sync>(&mut self, input: Option<I>) {
        self.input_device = match input {
            Some(device) => Some(Box::new(device)),
            None => None,
        }
    }

    pub fn add_output_device<O: OutputDevice + 'a + Send + Sync>(&mut self, output: O) {
        self.output_devices.push(Box::new(output));
    }

    pub fn start(&mut self) {
        if !self.running {
            if let Some(input_device) = &self.input_device {
                input_device.start(RoomControllerInputHandle::new(self.rc_lock.clone()));
            }

            for device in &self.output_devices {
                device.start(RoomControllerOutputHandle::new(self.rc_lock.clone()));
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(input_device) = &mut self.input_device {
            input_device.stop();
        }
        self.running = false;
    }
}
