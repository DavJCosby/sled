use std::sync::{Arc, RwLock};

use crate::room_controller::RoomController;

pub trait SpatialInputDriver {
    fn start(self, controller: Arc<RwLock<RoomController>>);
    fn stop(&mut self);
}

pub trait OutputDevice {
    fn start(&self, controller: Arc<RwLock<RoomController>>);
}
