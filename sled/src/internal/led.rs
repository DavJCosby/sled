use crate::internal::color;
use color::Rgb;
use glam::Vec2;

#[derive(Clone)]
pub struct Led {
    pub color: Rgb,
    position: Vec2,
    index: usize,
    segment: usize,
}

impl Led {
    pub fn new(color: Rgb, position: Vec2, index: usize, segment: usize) -> Self {
        Led {
            color,
            position,
            index,
            segment,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn segment(&self) -> usize {
        self.segment
    }
}
