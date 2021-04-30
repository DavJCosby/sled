#![crate_name = "doc"]

/// Settings and data for a room
pub struct RoomConfig {
    /// number of leds / meter
    pub led_density: f32,
    /// expected position of the observer (meters, meters)
    pub view_pos: Point,
    /// expected rotation of the observer in degrees
    pub view_rot: f32,
    pub strips: Vec<Strip>,
    pub leds: Vec<Color>,
}

impl RoomConfig {
    /// returns the number of pixels in the light strip chain, derived from [`RoomConfig`].led_density
    pub fn num_leds(self) -> usize {
        let mut count = 0.0;
        for w in self.strips {
            count += w.len() * self.led_density;
        }
        return count as usize;
    }
}

pub type Point = (f32, f32);
pub type Vector2D = Point;

/// LED light strip stretching from strip.0 to strip.1. Does not own any leds, see [`RoomConfig`].leds
pub type LineSegment = (Point, Point);
pub type Strip = LineSegment;

pub trait LineSegmentTrait {
    fn len(&self) -> f32;
    fn intersects(&self, other: &LineSegment) -> Option<Point>;
}

impl LineSegmentTrait for LineSegment {
    fn len(&self) -> f32 {
        let dx = self.1 .0 - self.0 .0;
        let dy = self.1 .1 - self.0 .1;
        return (dx * dx + dy * dy).sqrt();
    }

    fn intersects(&self, other: &LineSegment) -> Option<Point> {
        let a1 = self.1 .1 - self.0 .1;
        let b1 = self.0 .0 - self.1 .0;
        let c1 = a1 * self.0 .0 + b1 * self.0 .1;

        let a2 = other.1 .1 - other.0 .1;
        let b2 = other.0 .0 - other.1 .0;
        let c2 = a2 * other.0 .0 + b2 * other.0 .1;

        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        Some(((b2 * c1 - b1 * c2) / delta, (a1 * c2 - a2 * c1) / delta))
    }
}

///24-bit color tuple struct
pub type Color = (u8, u8, u8);
