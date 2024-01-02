use std::collections::HashSet;

use crate::{
    color::Rgb,
    led::Led,
    sled::{Filter, Sled},
};
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
                angle += std::f32::consts::TAU;
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

/// Filters
impl Sled {
    pub fn filter(&self, filter: impl Fn(&Led) -> bool) -> Filter {
        let filtered: HashSet<usize> = self
            .leds
            .iter()
            .filter_map(|led| if filter(led) { Some(led.index()) } else { None })
            .collect();
        filtered.into()
    }

    pub fn filter_by_angle(&self, angle_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| angle_filter(led.angle()))
    }

    pub fn filter_by_dir(&self, dir_filter: impl Fn(Vec2) -> bool) -> Filter {
        self.filter(|led| dir_filter(led.direction()))
    }

    pub fn filter_by_pos(&self, pos_filter: impl Fn(Vec2) -> bool) -> Filter {
        self.filter(|led| pos_filter(led.position()))
    }

    pub fn filter_by_dist(&self, dist_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| dist_filter(led.distance()))
    }

    pub fn filter_by_dist_from(&self, pos: Vec2, dist_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| {
            let dist = pos.distance(led.position());
            dist_filter(dist)
        })
    }
}
