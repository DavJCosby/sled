use crate::internal::color;
use color::Rgb;
use glam::Vec2;

#[derive(Clone)]
pub struct Led {
    pub color: Rgb,
    position: Vec2,
    direction: Vec2,
    angle: f32,
    index: usize,
    segment: usize,
}

impl Led {
    pub fn new(color: Rgb, position: Vec2, direction: Vec2, index: usize, segment: usize) -> Self {
        let angle = direction.angle_between(Vec2::new(0.0, 1.0));
        Led {
            color,
            position,
            direction,
            angle,
            index,
            segment,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn segment(&self) -> usize {
        self.segment
    }
}
