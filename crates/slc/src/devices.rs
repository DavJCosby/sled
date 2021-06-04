use std::sync::{Arc, RwLock};

use crate::room_controller::RoomController;

/// Trait for anything that should send input to the [RoomController](../room_controller/struct.RoomController.html).
pub trait InputDevice {
    /// Tells the driver to start sending input to the [RoomController](../room_controller/struct.RoomController.html).
    fn start(&self, controller: Arc<RwLock<RoomController>>);
        /// Tells the driver to stop sending input to the [RoomController](../room_controller/struct.RoomController.html).
    fn stop(&mut self);
}

/// Trait for anything that reads from the [RoomController](../room_controller/struct.RoomController.html).
pub trait OutputDevice {
    /// Tells the driver to reading data from the [RoomController](../room_controller/struct.RoomController.html).
    fn start(&self, controller: Arc<RwLock<RoomController>>);

}
