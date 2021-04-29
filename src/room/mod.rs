#![crate_name = "doc"]

pub struct RoomConfig {
    /// number of leds / meter
    pub led_density: f32,
    /// expected position of the observer (meters, meters)
    pub view_pos: Point,
    /// expected rotation of the observer in degrees
    pub view_rot: f32,
    pub walls: Vec<Wall>,
    pub pixels: Vec<Color>,
}

impl RoomConfig {
    /// returns the number of pixels in the light strip chain, derived from led_density.
    pub fn num_leds(self) -> usize {
        let mut count = 0.0;
        for w in self.walls {
            count += w.len() * self.led_density;
        }
        return count as usize;
    }
}

pub struct Wall(pub Point, pub Point);

impl Wall {
    pub fn len(&self) -> f32 {
        let dx = self.1 .0 - self.0 .0;
        let dy = self.1 .1 - self.0 .1;
        return (dx * dx + dy * dy).sqrt();
    }
}

pub struct Point(pub f32, pub f32);
pub struct Color(pub u8, pub u8, pub u8);
