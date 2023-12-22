use crate::color::Rgb;
use glam::Vec2;

use std::f32::consts::TAU;

#[derive(Clone)]

/// An LED in our Sled configuration, representing both the color of the LED as well as it's spatial information.
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
    /// Constructs an LED struct.
    /// Fields like `position`, `angle`, and `distance` are derived from `center_point`.
    pub fn new(
        color: Rgb,
        position: Vec2,
        index: usize,
        segment: usize,
        center_point: Vec2,
    ) -> Self {
        let direction = (position - center_point).normalize();

        let mut angle = direction.angle_between(Vec2::new(1.0, 0.0));
        if angle < 0.0 {
            angle = TAU + angle;
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

    /// Returns the position of the Led in world space.
    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// Returns the direction from the Sled's `center_point` to this Led. A normalized vector.
    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    /// Returns the angle from the Sled's `center_point` to this Led in radians.
    /// The direction `(1, 0)` is considered 0 radians, `(0, -1)` is pi/2 radian.
    pub fn angle(&self) -> f32 {
        self.angle
    }

    /// Returns the distance from the Sled's `center_point` to this Led.
    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Returns the index of the Led, keeping in mind that Leds in a Sled are treated in memory as one continuous strip.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the index of the LineSegment this Led belongs to.
    pub fn segment(&self) -> usize {
        self.segment
    }
}

impl PartialEq for Led {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl Eq for Led {}

impl std::hash::Hash for Led {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}
