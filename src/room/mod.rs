#![crate_name = "doc"]
use std::fs;
/// Settings and data for a room
pub struct Room {
    /// number of leds / meter
    pub led_density: f32,
    /// expected position of the observer (meters, meters)
    pub view_pos: Point,
    /// expected rotation of the observer in degrees
    pub view_rot: f32,
    /// collection of [`LineSegment`]s that represent LED strips
    pub strips: Vec<Strip>,
    pub leds: Vec<Color>,
}

impl Room {
    pub fn new(led_density: f32, view_pos: Point, view_rot: f32) -> Self {
        Room {
            led_density,
            view_pos,
            view_rot,
            strips: vec![],
            leds: vec![],
        }
    }

    pub fn new_from_file(filepath: &str) -> Self {
        let contents = fs::read_to_string(filepath).expect("something went wrong reading the file");
        let lines = contents.lines().collect::<Vec<&str>>();

        let led_density = lines[0].parse::<f32>().unwrap();
        let coords_str = lines[1].split(" ").collect::<Vec<&str>>();
        let view_pos = (
            coords_str[0].parse::<f32>().unwrap(),
            coords_str[1].parse::<f32>().unwrap(),
        );
        let view_rot = lines[2].parse::<f32>().unwrap();
        let mut strips: Vec<Strip> = vec![];

        for i in 3..lines.len() {
            let coords_str = lines[i].split(" ").collect::<Vec<&str>>();
            let p0x = coords_str[0].parse::<f32>().unwrap();
            let p0y = coords_str[1].parse::<f32>().unwrap();
            let p1x = coords_str[2].parse::<f32>().unwrap();
            let p1y = coords_str[3].parse::<f32>().unwrap();

            strips.push(((p0x, p0y), (p1x, p1y)));
        }

        let mut return_value = Room {
            led_density,
            view_pos,
            view_rot,
            strips,
            leds: vec![],
        };

        return_value.leds = vec![(0, 0, 0); return_value.num_leds()];
        return return_value;
    }

    /// returns the number of pixels in the light strip chain, derived from [`Room`].led_density
    pub fn num_leds(&self) -> usize {
        if self.leds.len() != 0 {
            return self.leds.len();
        }

        let mut count = 0.0;
        let strips = &self.strips;
        for w in strips {
            count += w.len() * self.led_density;
        }
        return count as usize;
    }
}

/// Point with x = point.0 and y = point.1
pub type Point = (f32, f32);
/// Vector2D with x = vec2d.0 and y = vec2d.1
pub type Vector2D = Point;

/// LED light strip stretching from strip.0 to strip.1. Does not own any leds, see [`Room`].leds
pub type LineSegment = (Point, Point);
/// LED light strip stretching from strip.0 to strip.1. Does not own any leds, see [`Room`].leds
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

    /// returns the point of intersection between two line segments, if there is one. Stolen from `<https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect>`
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

///24-bit color tuple alias
pub type Color = (u8, u8, u8);
