use std::sync::{Arc, RwLock};

use slc_core::room_controller::RoomController;

pub trait SpatialInputDriver {
    fn start(self, controller: Arc<RwLock<RoomController>>);
    fn stop(&mut self);
}
