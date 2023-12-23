use std::collections::HashSet;

use crate::{
    color::Rgb,
    error::SledError,
    led::Led,
    sled::{Set, Sled},
};
use glam::Vec2;
use smallvec::{smallvec, SmallVec};

/// position-based read and write methods
impl Sled {
    /* closest getters/setters */

    pub fn get_index_of_closest_to(&self, pos: Vec2) -> usize {
        // get the closest point on each segment and bundle relevant info,
        // then find the closest of those points
        let (alpha, _dist_sq, segment_index) = self
            .line_segments
            .iter()
            .enumerate()
            .map(|(index, segment)| {
                let (closest, alpha) = segment.closest_to_point(pos);
                let dist_sq = closest.distance_squared(pos);
                (alpha, dist_sq, index)
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        self.alpha_to_index(alpha, segment_index)
    }

    pub fn get_closest(&self) -> &Led {
        &self.leds[self.index_of_closest]
    }

    pub fn get_closest_to(&self, pos: Vec2) -> &Led {
        let index_of_closest = self.get_index_of_closest_to(pos);
        &self.leds[index_of_closest]
    }

    pub fn modulate_closest<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        let led = &mut self.leds[self.index_of_closest];
        led.color = color_rule(led);
    }

    pub fn modulate_closest_to<F: Fn(&Led) -> Rgb>(&mut self, pos: Vec2, color_rule: F) {
        let index_of_closest = self.get_index_of_closest_to(pos);
        let led = &mut self.leds[index_of_closest];
        led.color = color_rule(led);
    }

    pub fn set_closest(&mut self, color: Rgb) {
        self.leds[self.index_of_closest].color = color;
    }

    pub fn set_closest_to(&mut self, pos: Vec2, color: Rgb) {
        let index_of_closest = self.get_index_of_closest_to(pos);
        self.leds[index_of_closest].color = color;
    }

    /* furthest getters/setters */

    pub fn get_index_of_furthest_from(&self, pos: Vec2) -> usize {
        let (index_of_furthest, _dist) = self
            .vertex_indices
            .iter()
            .map(|i| {
                let vertex_pos = self.leds[*i].position();
                (*i, pos.distance_squared(vertex_pos))
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        index_of_furthest
    }

    pub fn get_furthest(&self) -> &Led {
        &self.leds[self.index_of_furthest]
    }

    pub fn get_furthest_from(&self, pos: Vec2) -> &Led {
        let index_of_furthest = self.get_index_of_furthest_from(pos);
        &self.leds[index_of_furthest]
    }

    pub fn modulate_furthest<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        let led = &mut self.leds[self.index_of_furthest];
        led.color = color_rule(led);
    }

    pub fn modulate_furthest_from<F: Fn(&Led) -> Rgb>(&mut self, pos: Vec2, color_rule: F) {
        let index_of_furthest = self.get_index_of_furthest_from(pos);
        let led = &mut self.leds[index_of_furthest];
        led.color = color_rule(led);
    }

    pub fn set_furthest(&mut self, color: Rgb) {
        self.leds[self.index_of_furthest].color = color;
    }

    pub fn set_furthest_from(&mut self, pos: Vec2, color: Rgb) {
        let index_of_furthest = self.get_index_of_furthest_from(pos);
        self.leds[index_of_furthest].color = color;
    }

    /* at distance methods */

    fn indices_at_dist(&self, pos: Vec2, dist: f32) -> SmallVec<[usize; 8]> {
        let mut all_at_distance = smallvec![];
        for (segment_index, segment) in self.line_segments.iter().enumerate() {
            for alpha in segment.intersects_circle(pos, dist) {
                let index = self.alpha_to_index(alpha, segment_index);
                all_at_distance.push(index);
            }
        }

        all_at_distance
    }

    pub fn get_at_dist(&self, dist: f32) -> Set {
        self.get_at_dist_from(dist, self.center_point)
    }

    pub fn get_at_dist_from(&self, dist: f32, pos: Vec2) -> Set {
        let mut all_at_distance = HashSet::new();

        for (segment_index, segment) in self.line_segments.iter().enumerate() {
            for alpha in segment.intersects_circle(pos, dist) {
                let index = self.alpha_to_index(alpha, segment_index);
                all_at_distance.insert(index);
            }
        }

        all_at_distance.into()
    }

    pub fn modulate_at_dist<F: Fn(&Led) -> Rgb>(
        &mut self,
        dist: f32,
        color_rule: F,
    ) -> Result<(), SledError> {
        self.modulate_at_dist_from(dist, self.center_point, color_rule)
    }

    pub fn modulate_at_dist_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dist: f32,
        pos: Vec2,
        color_rule: F,
    ) -> Result<(), SledError> {
        let indices = self.indices_at_dist(pos, dist);

        if indices.is_empty() {
            return Err(SledError {
                message: format!("No LEDs exist at a distance of {} from {}", dist, pos),
            });
        }

        for i in indices {
            let led = &mut self.leds[i];
            led.color = color_rule(led);
        }

        Ok(())
    }

    pub fn set_at_dist(&mut self, dist: f32, color: Rgb) -> Result<(), SledError> {
        self.set_at_dist_from(dist, self.center_point, color)
    }

    pub fn set_at_dist_from(&mut self, dist: f32, pos: Vec2, color: Rgb) -> Result<(), SledError> {
        let indices = self.indices_at_dist(pos, dist);

        if indices.is_empty() {
            return Err(SledError {
                message: format!(
                    "No LEDs exist at a distance of {} from the center point.",
                    dist
                ),
            });
        }

        for index in indices {
            self.leds[index].color = color;
        }

        Ok(())
    }

    /* within distance methods */

    pub fn get_within_dist(&self, dist: f32) -> Set {
        self.get_within_dist_from(dist, self.center_point)
    }

    pub fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Set {
        let mut all_within_distance = HashSet::new();

        for (segment_index, segment) in self.line_segments.iter().enumerate() {
            let intersections = segment.intersects_solid_circle(pos, dist);
            let first = intersections.first();
            let second = intersections.get(1);

            if first.is_some() && second.is_some() {
                let first = self.alpha_to_index(*first.unwrap(), segment_index);
                let second = self.alpha_to_index(*second.unwrap(), segment_index);
                let range = first.min(second)..first.max(second);
                all_within_distance.extend(range);
            }
        }

        all_within_distance.into()
    }

    pub fn modulate_within_dist<F: Fn(&Led) -> Rgb>(&mut self, dist: f32, color_rule: F) {
        for led in &mut self.leds {
            if led.distance() < dist {
                led.color = color_rule(led);
            }
        }
    }

    pub fn set_within_dist(&mut self, dist: f32, color: Rgb) {
        for led in &mut self.leds {
            if led.distance() < dist {
                led.color = color;
            }
        }
    }

    pub fn modulate_within_dist_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dist: f32,
        pos: Vec2,
        color_rule: F,
    ) {
        let target_sq = dist.powi(2);

        for led in &mut self.leds {
            if led.position().distance_squared(pos) < target_sq {
                led.color = color_rule(led);
            }
        }
    }

    pub fn set_within_dist_from(&mut self, dist: f32, pos: Vec2, color: Rgb) {
        let target_sq = dist.powi(2);

        for led in &mut self.leds {
            if led.position().distance_squared(pos) < target_sq {
                led.color = color;
            }
        }
    }
}
