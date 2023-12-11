use crate::color::Rgb;
use glam::Vec2;
use std::f32::consts::PI;

#[derive(Clone)]

pub struct Led {
    pub color: Rgb,
    position: Vec2,
    direction: Vec2,
    angle: f32,
    distance: f32,
    index: usize,
    segment: usize,
}

impl Led {
    pub fn new(
        color: Rgb,
        position: Vec2,
        direction: Vec2,
        index: usize,
        segment: usize,
        center_point: Vec2,
    ) -> Self {
        let mut angle = direction.angle_between(Vec2::new(1.0, 0.0));
        if angle < 0.0 {
            angle = (2.0 * PI) + angle;
        }

        let distance = position.distance(center_point);
        
        Led {
            color,
            position,
            direction,
            angle,
            distance,
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

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn segment(&self) -> usize {
        self.segment
    }
}
