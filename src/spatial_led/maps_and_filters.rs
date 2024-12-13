use alloc::collections::BTreeSet;

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

use crate::{
    color::ColorType,
    led::Led,
    spatial_led::{Filter, Sled},
    Vec2,
};

/// Maps
impl<Color: ColorType> Sled<Color> {
    pub fn map(&mut self, led_to_color_map: impl Fn(&Led<Color>) -> Color) {
        self.leds
            .iter_mut()
            .for_each(|led| led.color = led_to_color_map(led));
    }

    pub fn map_by_index(&mut self, index_to_color_map: impl Fn(usize) -> Color) {
        self.map(|led| index_to_color_map(led.index() as usize));
    }

    pub fn map_by_segment(&mut self, segment_index_to_color_map: impl Fn(usize) -> Color) {
        self.map(|led| segment_index_to_color_map(led.segment() as usize));
    }

    pub fn map_by_pos(&mut self, pos_to_color_map: impl Fn(Vec2) -> Color) {
        self.map(|led| pos_to_color_map(led.position()));
    }

    pub fn map_by_dir(&mut self, dir_to_color_map: impl Fn(Vec2) -> Color) {
        self.map(|led| dir_to_color_map(led.direction()));
    }

    pub fn map_by_dir_from(&mut self, point: Vec2, dir_to_color_map: impl Fn(Vec2) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let dir = (point - led.position()).normalize_or_zero();
            led.color = dir_to_color_map(dir)
        });
    }

    pub fn map_by_angle(&mut self, angle_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            led.color = angle_to_color_map(led.angle());
        });
    }

    pub fn map_by_angle_from(&mut self, point: Vec2, angle_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let delta = point - led.position();
            let angle = delta.x.atan2(delta.y);
            led.color = angle_to_color_map(angle);
        });
    }

    pub fn map_by_dist(&mut self, dist_to_color_map: impl Fn(f32) -> Color) {
        self.leds
            .iter_mut()
            .for_each(|led| led.color = dist_to_color_map(led.distance()));
    }

    pub fn map_by_dist_from(&mut self, pos: Vec2, dist_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let dist = pos.distance(led.position());
            led.color = dist_to_color_map(dist);
        });
    }
}

/// Filters
impl<Color: ColorType> Sled<Color> {
    pub fn filter(&self, filter: impl Fn(&Led<Color>) -> bool) -> Filter {
        let filtered: BTreeSet<u16> = self
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
