#![crate_name = "doc"]
use crate::util::*;
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
