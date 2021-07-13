/// Point with x = point.0 and y = point.1.
pub type Point = (f32, f32);
/// Vector with x = point.0 and y = point.1.
pub type Vector2D = Point;
///24-bit color tuple alias.
pub type Color = (u8, u8, u8);

/// Line segment stretching from line_segment.0 to line_segment.1.
pub type LineSegment = (Point, Point);
/// LED light strip stretching from strip.0 to strip.1. Does not own any leds, see [Room](../room/struct.Room.html).leds.
pub type Strip = LineSegment;

pub trait LineSegmentTrait {
    fn len(&self) -> f32;
    fn intersects(&self, other: &LineSegment) -> Option<Point>;
    fn lerp(&self, t: f32) -> Point;
}

impl LineSegmentTrait for LineSegment {
    /// Returns the length of the LineSegment, obtained using the pythagorean theorem.
    fn len(&self) -> f32 {
        let dx = self.1 .0 - self.0 .0;
        let dy = self.1 .1 - self.0 .1;
        return (dx * dx + dy * dy).sqrt();
    }

    /// Interpolates between line_segment.0 and line_segment.1 at `t`.
    fn lerp(&self, t: f32) -> Point {
        (
            self.0 .0 + (self.1 .0 - self.0 .0) * t,
            self.0 .1 + (self.1 .1 - self.0 .1) * t,
        )
    }

    /// Returns the point of intersection between two LineSegments, if there is one.
    fn intersects(&self, other: &LineSegment) -> Option<Point> {
        let s1_x = self.1 .0 - self.0 .0;
        let s1_y = self.1 .1 - self.0 .1;
        let s2_x = other.1 .0 - other.0 .0;
        let s2_y = other.1 .1 - other.0 .1;

        let denom = 1.0 / (-s2_x * s1_y + s1_x * s2_y);

        let s = (-s1_y * (self.0 .0 - other.0 .0) + s1_x * (self.0 .1 - other.0 .1)) * denom;
        let t = (s2_x * (self.0 .1 - other.0 .1) - s2_y * (self.0 .0 - other.0 .0)) * denom;

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            // collision detected
            return Some((self.0 .0 + (t * s1_x), self.0 .1 + (t * s1_y)));
        } else {
            None
        }
    }
}
