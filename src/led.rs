use glam::Vec2;

use crate::color::ColorType;

#[derive(Copy, Clone)]

/// An LED in our Sled configuration, representing both the color of the LED as well as it's spatial information.
pub struct Led<Color: ColorType> {
    pub color: Color,
    position: Vec2,
    angle: f32,
    distance: f32,
    index: u16,
    segment: u8,
}

/// *All properties listed below are pre-calculated on construction;
/// there is no substantial overhead for calling these methods.*
impl<Color: ColorType> Led<Color> {
    /// Constructs an LED struct.
    /// Fields like `position`, `angle`, and `distance` are derived from `center_point`.
    pub(crate) fn new(
        color: Color,
        position: Vec2,
        index: u16,
        segment: u8,
        center_point: Vec2,
    ) -> Self {
        let offset = position - center_point;
        let angle = offset.y.atan2(offset.x);
        let distance = offset.length();
        // let mut angle = direction.angle_between(Vec2::new(1.0, 0.0));
        // if angle < 0.0 {
        //     angle += TAU;
        // }
        // let distance = position.distance(center_point);

        Led {
            color,
            position,
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
        Vec2::new(self.angle.cos(), self.angle.sin())
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
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Returns the index of the LineSegment this Led belongs to.
    pub fn segment(&self) -> u8 {
        self.segment
    }
}

impl<Color: ColorType> PartialEq for Led<Color> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl<Color: ColorType> Eq for Led<Color> {}

impl<Color: ColorType> PartialOrd for Led<Color> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Color: ColorType> Ord for Led<Color> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index())
    }
}

impl<Color: ColorType> std::hash::Hash for Led<Color> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl<Color: ColorType> std::fmt::Debug for Led<Color> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = self.direction();
        f.debug_struct("Led")
            .field("color", &self.color)
            .field("position", &(self.position.x, self.position.y))
            .field("direction", &(dir.x, dir.y))
            .field("angle", &self.angle)
            .field("distance", &self.distance)
            .field("index", &self.index)
            .field("segment", &self.segment)
            .finish()
    }
}

impl<Color: ColorType> std::fmt::Display for Led<Color> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.index, self.color)
    }
}
