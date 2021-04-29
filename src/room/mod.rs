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

/// LED light strip stretching from strip.0 to strip.1. Does not own any leds, see [`RoomConfig`].leds
pub struct Strip(pub Point, pub Point);

impl Strip {
    pub fn len(&self) -> f32 {
        let dx = self.1 .0 - self.0 .0;
        let dy = self.1 .1 - self.0 .1;
        return (dx * dx + dy * dy).sqrt();
    }
}

/// Barebones 2d tuple struct
pub struct Point(pub f32, pub f32);

///24-bit color tuple struct
pub struct Color(pub u8, pub u8, pub u8);
