use crate::util::*;
use std::fs;

/// Settings and data for a Room, to be consumed by a [RoomController](../room_controller/struct.RoomController.html).
pub struct Room {
    /// Number of leds / meter.
    density: f32,
    /// expected position of the observer (meters, meters).
    view_pos: Point,
    /// expected rotation of the observer in degrees (automaticallly converted to radians).
    view_rot: f32,
    /// collection of [LineSegments](LineSegment) that represent LED strips.
    strips: Vec<Strip>,
    /// list of all led colors in the room. Vector size should be `length of all strips * density`.
    leds: Vec<Color>,
    /// Brightness of the leds on a 0-255 scale.
    pub brightness: u8,
}

impl Room {
    /// constructs a room from a .rcfg file.
    pub fn new_from_file(filepath: &str) -> Self {
        let contents = fs::read_to_string(filepath).expect("something went wrong reading the file");
        let lines = contents.lines().collect::<Vec<&str>>();

        let density = lines[0].parse::<f32>().unwrap();
        let coords_str = lines[1].split(" ").collect::<Vec<&str>>();
        let view_pos = (
            coords_str[0].parse::<f32>().unwrap(),
            coords_str[1].parse::<f32>().unwrap(),
        );
        let view_rot = lines[2].parse::<f32>().unwrap().to_radians();
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
            density,
            view_pos,
            view_rot,
            strips,
            leds: vec![],
            brightness: 255,
        };

        return_value.leds = vec![(0, 0, 0); return_value.num_expected_leds()];
        return return_value;
    }

    /// read-only access to the density field.
    pub fn density(&self) -> f32 {
        self.density
    }

    /// read-only access to the view_pos field.
    pub fn view_pos(&self) -> Point {
        self.view_pos
    }

    /// read-only access to the view_rot field.
    pub fn view_rot(&self) -> f32 {
        self.view_rot
    }

    /// read-only access to the srips field.
    pub fn strips(&self) -> &Vec<Strip> {
        &self.strips
    }

    /// read-only access to the leds field.
    pub fn leds(&self) -> &Vec<Color> {
        &self.leds
    }

    /// Transforms a direction in view space to room space.
    pub fn view_dir_to_room_dir(&self, view_dir: Vector2D) -> Vector2D {
        let view_angle = view_dir.1.atan2(view_dir.0);
        let room_rot = self.view_angle_to_room_angle(view_angle);
        return (room_rot.cos(), room_rot.sin());
    }

    /// Transforms an angle in view space to room space.
    pub fn view_angle_to_room_angle(&self, view_angle: f32) -> f32 {
        return self.view_rot + view_angle;
    }

    /// Interpolates down the chain of strips to return the point at t.
    pub fn get_pos_at_t(&self, t: f32) -> Point {
        // find sum of strip lengths
        let mut sum = 0.0;
        for strip in &self.strips {
            sum += strip.len();
        }
        // find matching strip
        let mut cur_dist = 0.0;
        let mut target_strip = self.strips.get(0).unwrap();

        for strip in &self.strips {
            let next_dist = cur_dist + strip.len();
            if next_dist > t * sum {
                target_strip = &strip;
                break;
            }
            cur_dist = next_dist;
        }

        // find point along that strip
        let leftover_t = (t - (cur_dist / sum)) / (target_strip.len() / sum);
        return target_strip.lerp(leftover_t);
    }

    /// Sets the led at index to the given color.
    pub fn set_led(&mut self, index: usize, color: Color) {
        self.leds[index] = color;
    }

    /// Returns the number of leds expected in the room, calulated using strip lengths and density.
    fn num_expected_leds(&self) -> usize {
        let mut count = 0.0;
        let strips = &self.strips;
        for w in strips {
            count += w.len() * self.density;
        }
        return count as usize;
    }
}
