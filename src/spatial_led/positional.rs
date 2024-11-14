use alloc::collections::BTreeSet;

use crate::{
    color::Rgb,
    led::Led,
    spatial_led::{Filter, Sled},
};

use glam::Vec2;
use smallvec::{smallvec, SmallVec};

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

/// # position-based read and write methods
impl Sled {
    /* closest getters/setters */

    /// Returns the index of the [LED](Led) closest to a given point.
    ///
    /// O(SEGMENTS)
    ///
    pub fn index_of_closest_to(&self, pos: Vec2) -> usize {
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

    /// Returns the [LED](Led) closest to the center point.
    ///
    /// O(1)
    pub fn closest(&self) -> &Led {
        &self.leds[self.index_of_closest]
    }

    /// Returns the [LED](Led) closest to a given point.
    ///
    /// O(SEGMENTS)
    pub fn closest_to(&self, pos: Vec2) -> &Led {
        let index_of_closest = self.index_of_closest_to(pos);
        &self.leds[index_of_closest]
    }

    /// Modulates the color of the [LED](Led) closest to the center point.
    ///
    /// O(1)
    ///  
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_closest(|led| led.color + Rgb::new(0.2, 0.2, 0.2));
    ///# Ok(())
    ///# }
    pub fn modulate_closest<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        let led = &mut self.leds[self.index_of_closest];
        led.color = color_rule(led);
    }

    /// Modulates the color of the [LED](Led) closest to a given point.
    ///
    /// O(SEGMENTS)
    ///  
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_closest_to(Vec2::new(0.5, 0.0), |led| {
    ///     led.color + Rgb::new(0.2, 0.2, 0.2)
    /// });
    ///# Ok(())
    ///# }
    pub fn modulate_closest_to<F: Fn(&Led) -> Rgb>(&mut self, pos: Vec2, color_rule: F) {
        let index_of_closest = self.index_of_closest_to(pos);
        let led = &mut self.leds[index_of_closest];
        led.color = color_rule(led);
    }

    /// Sets the color of the [LED](Led) closest to the center point.
    ///
    /// O(1)
    pub fn set_closest(&mut self, color: Rgb) {
        self.leds[self.index_of_closest].color = color;
    }

    /// Sets the color of the [LED](Led) closest to a given point.
    ///
    /// O(SEGMENTS)
    pub fn set_closest_to(&mut self, pos: Vec2, color: Rgb) {
        let index_of_closest = self.index_of_closest_to(pos);
        self.leds[index_of_closest].color = color;
    }

    /* furthest getters/setters */

    /// Returns the index of the [LED](Led) furthest from a given point.
    ///
    /// O(VERTICES)
    pub fn index_of_furthest_from(&self, pos: Vec2) -> usize {
        // get the distance_squared of each vertex point, then find out which is the furthest.
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

    /// Returns the index of the [LED](Led) furthest from the center point.
    ///
    /// O(1)
    pub fn index_of_furthest(&self) -> usize {
        self.index_of_furthest
    }
    /// Returns the [LED](Led) furthest from the center point.
    ///
    /// O(1)
    pub fn furthest(&self) -> &Led {
        &self.leds[self.index_of_furthest]
    }

    /// Returns the [LED](Led) furthest from a given point.
    ///
    /// O(VERTICES)
    pub fn furthest_from(&self, pos: Vec2) -> &Led {
        let index_of_furthest = self.index_of_furthest_from(pos);
        &self.leds[index_of_furthest]
    }

    /// Modulates the color of the [LED](Led) furthest from the center point.
    ///
    /// O(1)
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_furthest(|led| led.color / led.distance());
    ///# Ok(())
    ///# }
    pub fn modulate_furthest<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        let led = &mut self.leds[self.index_of_furthest];
        led.color = color_rule(led);
    }

    /// Modulates the color of the [LED](Led) furthest from a given point
    ///
    /// O(SEGMENTS)
    ///  
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_furthest_from(Vec2::new(0.0, -1.0), |led| {
    ///     led.color - Rgb::new(0.2, 0.2, 0.2)
    /// });
    ///# Ok(())
    ///# }
    pub fn modulate_furthest_from<F: Fn(&Led) -> Rgb>(&mut self, pos: Vec2, color_rule: F) {
        let index_of_furthest = self.index_of_furthest_from(pos);
        let led = &mut self.leds[index_of_furthest];
        led.color = color_rule(led);
    }

    /// Sets the color of the [LED](Led) furthest from the center point.
    ///
    /// O(1)
    pub fn set_furthest(&mut self, color: Rgb) {
        self.leds[self.index_of_furthest].color = color;
    }

    /// Sets the color of the [LED](Led) furthest from a given point.
    ///
    /// O(VERTICES)
    pub fn set_furthest_from(&mut self, pos: Vec2, color: Rgb) {
        let index_of_furthest = self.index_of_furthest_from(pos);
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

    pub fn at_dist(&self, dist: f32) -> Filter {
        self.at_dist_from(dist, self.center_point)
    }

    pub fn at_dist_from(&self, dist: f32, pos: Vec2) -> Filter {
        let mut all_at_distance = BTreeSet::new();

        for (segment_index, segment) in self.line_segments.iter().enumerate() {
            for alpha in segment.intersects_circle(pos, dist) {
                let index = self.alpha_to_index(alpha, segment_index);
                all_at_distance.insert(index as u16);
            }
        }

        all_at_distance.into()
    }

    pub fn modulate_at_dist<F: Fn(&Led) -> Rgb>(&mut self, dist: f32, color_rule: F) -> bool {
        self.modulate_at_dist_from(dist, self.center_point, color_rule)
    }

    pub fn modulate_at_dist_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dist: f32,
        pos: Vec2,
        color_rule: F,
    ) -> bool {
        let indices = self.indices_at_dist(pos, dist);
        let anything_found = !indices.is_empty();
        for i in indices {
            let led = &mut self.leds[i];
            led.color = color_rule(led);
        }

        anything_found
    }

    pub fn set_at_dist(&mut self, dist: f32, color: Rgb) -> bool {
        self.set_at_dist_from(dist, self.center_point, color)
    }

    pub fn set_at_dist_from(&mut self, dist: f32, pos: Vec2, color: Rgb) -> bool {
        let indices = self.indices_at_dist(pos, dist);
        let anything_found = !indices.is_empty();

        for index in indices {
            self.leds[index].color = color;
        }

        anything_found
    }

    /* within distance methods */

    pub fn within_dist(&self, dist: f32) -> Filter {
        self.within_dist_from(dist, self.center_point)
    }

    pub fn within_dist_from(&self, dist: f32, pos: Vec2) -> Filter {
        let mut all_within_distance = BTreeSet::new();

        let target_sq = dist.powi(2);

        for led in &self.leds {
            if led.position().distance_squared(pos) < target_sq {
                all_within_distance.insert(led.index());
            }
        }

        all_within_distance.into()
    }

    pub fn modulate_within_dist<F: Fn(&Led) -> Rgb>(&mut self, dist: f32, color_rule: F) -> bool {
        let mut changes_made = false;

        for led in &mut self.leds {
            if led.distance() < dist {
                led.color = color_rule(led);
                changes_made = true;
            }
        }

        changes_made
    }

    pub fn set_within_dist(&mut self, dist: f32, color: Rgb) -> bool {
        let mut changes_made = false;

        for led in &mut self.leds {
            if led.distance() < dist {
                led.color = color;
                changes_made = true;
            }
        }

        changes_made
    }

    pub fn modulate_within_dist_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dist: f32,
        pos: Vec2,
        color_rule: F,
    ) -> bool {
        let target_sq = dist.powi(2);
        let mut changes_made = false;

        for led in &mut self.leds {
            if led.position().distance_squared(pos) < target_sq {
                led.color = color_rule(led);
                changes_made = true
            }
        }

        changes_made
    }

    pub fn set_within_dist_from(&mut self, dist: f32, pos: Vec2, color: Rgb) -> bool {
        let target_sq = dist.powi(2);
        let mut changes_made = false;

        for led in &mut self.leds {
            if led.position().distance_squared(pos) < target_sq {
                led.color = color;
                changes_made = true;
            }
        }

        changes_made
    }
}
