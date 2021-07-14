use std::sync::{Arc, RwLock};

use crate::prelude::*;

pub struct Room<'a> {
    rc_lock: Arc<RwLock<RoomController>>,
    input_device: Option<Box<dyn InputDevice + 'a + Send + Sync>>,
    output_devices: Vec<Box<dyn OutputDevice + 'a + Send + Sync>>,
    running: bool,
}

impl<'a> Room<'a> {
    /// Constructs a Room from a room config (.rcfg) file.
    ///```rs
    /// 60                      // LED density (LEDs/meter)
    /// 0.75 0.75               // Position (meters) of the "Camera"
    /// 0                       // Rotational offset (degrees, counter-clockwise, auto converted into radians) of the "Camera" (0 = facing right, 90 = facing up)
    /// 0 -1.5 2.0 0            // strip0_start.x strip0_start.y strip0_end.x strip0_end.y
    /// 2.0 0 1.5 1.5           // strip1_start.x strip1_start.y strip1_end.x strip1_end.y
    /// 1.5 1.5 1.5 2.0         // strip2_start.x strip2_start.y strip0_end.x strip2_end.y
    ///                         // and so on...
    /// ```
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

    /// Changes the input device associated with the room. If an input device is already
    /// connected, its .stop() method is called first.
    pub fn set_input_device<I: InputDevice + 'a + Send + Sync>(&mut self, input: I) {
        if let Some(input_device) = &mut self.input_device {
            input_device.stop();
        }
        self.input_device = Some(Box::new(input));
    }

    /// Returns the current input device.
    pub fn get_input_device(&self) -> &Box<dyn InputDevice + 'a + Send + Sync> {
        return &self.input_device.as_ref().unwrap();
    }

    /// Adds an output device to the room.
    pub fn add_output_device<O: OutputDevice + 'a + Send + Sync>(&mut self, output: O) {
        self.output_devices.push(Box::new(output));
    }

    /// starts all connected input and output devices.
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

    /// stops all connected input devices.
    pub fn stop(&mut self) {
        if let Some(input_device) = &mut self.input_device {
            input_device.stop();
        }
        self.running = false;
    }
}
