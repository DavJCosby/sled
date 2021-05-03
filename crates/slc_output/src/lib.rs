use std::sync::{Arc, RwLock};

use slc_core::room_controller::RoomController;

pub trait OutputDevice {
    fn start(controller: Arc<RwLock<RoomController>>);
}
