use crate::internal::color;
use color::Rgb;
use glam::Vec2;

pub struct Led {
    pub color: Rgb,
    position: Vec2,
    segment_index: usize,
}

impl Led {
    pub fn new(color: Rgb, position: Vec2, segment_index: usize) -> Self {
        Led {
            color,
            position,
            segment_index,
        }
    }
}
