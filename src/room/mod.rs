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
        let lines = contents.split("\n").collect::<Vec<&str>>();

        let led_density = lines[0].parse::<f32>().unwrap();
        let coords_str = lines[1].split(" ").collect::<Vec<&str>>();
        let view_pos = (
            coords_str[0].parse::<f32>().unwrap(),
            coords_str[1].parse::<f32>().unwrap(),
        );
        let view_rot = lines[2].parse::<f32>().unwrap();
        let mut strips: Vec<Strip> = vec![];

        for i in 3..lines.len() - 1 {
            let coords_str = lines[i].split(" ").collect::<Vec<&str>>();
            let p0x = coords_str[0].parse::<f32>().unwrap();
            let p0y = coords_str[1].parse::<f32>().unwrap();
            let p1x = coords_str[2].parse::<f32>().unwrap();
            let p1y = coords_str[3].parse::<f32>().unwrap();

            strips.push(((p0x, p0y), (p1x, p1y)));
        }

        //println!("{}", lines);
        let num_leds = strips.len();
        RoomConfig {
        let mut return_value = Room {
            led_density,
            view_pos,
            view_rot,
            strips,
            leds: vec![(0, 0, 0); num_leds],
        }
    }

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
