use crate::color::Rgb;
use glam::Vec2;

use std::f32::consts::TAU;

#[derive(Clone)]

/// An LED in our Sled configuration, representing both the color of the LED as well as it's spatial information.
pub struct Led {
    pub color: Rgb,
    position: Vec2,
    offset: Vec2,
    index: u16,
    segment: u8,
}

/// *All properties listed below are pre-calculated on construction;
/// there is no substantial overhead for calling these methods.*
impl Led {
    /// Constructs an LED struct.
    /// Fields like `position`, `angle`, and `distance` are derived from `center_point`.
    pub(crate) fn new(
        color: Rgb,
        position: Vec2,
        index: u16,
        segment: u8,
        center_point: Vec2,
    ) -> Self {
        let offset = position - center_point;

        // let mut angle = direction.angle_between(Vec2::new(1.0, 0.0));
        // if angle < 0.0 {
        //     angle += TAU;
        // }
        // let distance = position.distance(center_point);

        Led {
            color,
            position,
            offset,
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
        self.offset.normalize()
    }

    /// Returns the angle from the Sled's `center_point` to this Led in radians.
    /// The direction `(1, 0)` is considered 0 radians, `(0, -1)` is pi/2 radian.
    pub fn angle(&self) -> f32 {
        return self.offset.x.atan2(self.offset.y);
        // let angle = Self::fast_angle_between(self.offset, Self::POSITIVE_X);
        // if angle < 0.0 {
        //     angle + TAU
        // } else {
        //     angle
        // }
    }

    /// Returns the distance from the Sled's `center_point` to this Led.
    pub fn distance(&self) -> f32 {
        self.offset.length()
    }

    pub fn distance_squared(&self) -> f32 {
        self.offset.length_squared()
    }

    /// Returns the index of the Led, keeping in mind that Leds in a Sled are treated in memory as one continuous strip.
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Returns the index of the LineSegment this Led belongs to.
    pub fn segment(&self) -> u8 {
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
