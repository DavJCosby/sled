use std::sync::{Arc, RwLock};

use slc_core::room_controller::RoomController;

pub trait OutputDevice {
    fn start(&self, controller: Arc<RwLock<RoomController>>);
}
