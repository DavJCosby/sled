use crate::{color::Rgb, led::Led, sled::Sled};
use glam::Vec2;

/// Maps
impl Sled {
    pub fn map(&mut self, led_to_color_map: impl Fn(&Led) -> Rgb) {
        // consider parallelizing, adding a map_parallel method, or making parallelization an opt-in compiler feature.
        for led in &mut self.leds {
            led.color = led_to_color_map(led);
        }
    }

    pub fn map_by_index(&mut self, index_to_color_map: impl Fn(usize) -> Rgb) {
        self.map(|led| index_to_color_map(led.index()));
    }

    pub fn map_by_segment(&mut self, segment_index_to_color_map: impl Fn(usize) -> Rgb) {
        self.map(|led| segment_index_to_color_map(led.segment()));
    }

    pub fn map_by_pos(&mut self, pos_to_color_map: impl Fn(Vec2) -> Rgb) {
        self.map(|led| pos_to_color_map(led.position()));
    }

    pub fn map_by_dir(&mut self, dir_to_color_map: impl Fn(Vec2) -> Rgb) {
        self.map(|led| dir_to_color_map(led.direction()));
    }

    pub fn map_by_dir_from(&mut self, point: Vec2, dir_to_color_map: impl Fn(Vec2) -> Rgb) {
        self.map(|led| {
            let dir = (point - led.position()).normalize_or_zero();
            dir_to_color_map(dir)
        });
    }

    pub fn map_by_angle(&mut self, angle_to_color_map: impl Fn(f32) -> Rgb) {
        self.map(|led| angle_to_color_map(led.angle()));
    }

    pub fn map_by_angle_from(&mut self, point: Vec2, angle_to_color_map: impl Fn(f32) -> Rgb) {
        let pos_x = Vec2::new(0.0, 1.0);
        self.map(|led| {
            let mut angle = (point - led.position()).angle_between(pos_x);
            if angle < 0.0 {
                angle = (2.0 * std::f32::consts::PI) + angle;
            }

            angle_to_color_map(angle)
        });
    }

    pub fn map_by_dist(&mut self, dist_to_color_map: impl Fn(f32) -> Rgb) {
        self.map(|led| dist_to_color_map(led.distance()));
    }

    pub fn map_by_dist_from(&mut self, pos: Vec2, dist_to_color_map: impl Fn(f32) -> Rgb) {
        self.map(|led| {
            let dist = pos.distance(led.position());
            dist_to_color_map(dist)
        });
    }
}
