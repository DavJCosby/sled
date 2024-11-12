use alloc::collections::BTreeSet;

use crate::{color::Rgb, led::Led, Filter, Sled};
use glam::Vec2;
use smallvec::SmallVec;

/// # directional read and write methods
impl Sled {
    fn raycast_for_indices(&self, start: Vec2, dir: Vec2) -> SmallVec<[usize; 4]> {
        let dist = 100_000.0;
        let end = start + dir * dist;

        let mut intersections = smallvec::smallvec![];
        for (seg_index, segment) in self.line_segments.iter().enumerate() {
            if let Some(t) = segment.intersects_line(start, end) {
                let index = self.alpha_to_index(t, seg_index);
                intersections.push(index);
            }
        }

        intersections
    }

    /* direction setters/getters */

    /// Returns A [Filter] containing each [LED](Led) in the given direction from the center point.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// If no LEDs exist at the given direction, the Filter will be empty.
    ///
    /// O(SEGMENTS)
    pub fn at_dir(&self, dir: Vec2) -> Filter {
        self.at_dir_from(dir, self.center_point)
    }

    /// Returns A [Filter] containing each [LED](Led) in the given direction from a given point.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Currently returns no more than 4 LEDs, may change in the future.
    pub fn at_dir_from(&self, dir: Vec2, pos: Vec2) -> Filter {
        let intersecting_indices = self.raycast_for_indices(pos, dir);
        intersecting_indices
            .iter()
            .map(|i| *i as u16)
            .collect::<BTreeSet<u16>>()
            .into()
    }

    /// Modulates the color of each [LED](Led) in the given direction from the center point.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED in that direction, true otherwise.
    ///
    /// O(SEGMENTS)
    ///
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_at_dir(Vec2::new(0.0, 1.0), |led| led.color * 2.0);
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_at_dir<F: Fn(&Led) -> Rgb>(&mut self, dir: Vec2, color_rule: F) -> bool {
        self.modulate_at_dir_from(dir, self.center_point, color_rule)
    }

    /// Modulates the color of each [LED](Led) in the given direction from a given point.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED in that direction, true otherwise.
    ///
    /// O(SEGMENTS)
    ///
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// let dir = Vec2::new(-1.0, 0.0);
    /// let from = Vec2::new(0.25, -0.6);
    /// sled.modulate_at_dir_from(dir, from, |led| {
    ///     led.color * 2.0
    /// });
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_at_dir_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dir: Vec2,
        pos: Vec2,
        color_rule: F,
    ) -> bool {
        let intersecting_indices = self.raycast_for_indices(pos, dir);

        if intersecting_indices.is_empty() {
            return false;
        }

        for index in intersecting_indices {
            let led = &mut self.leds[index];
            led.color = color_rule(led);
        }

        true
    }

    /// Sets the color of each [LED](Led) in the given direction from the center.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED in that direction, true otherwise.
    ///
    /// O(SEGMENTS)
    pub fn set_at_dir(&mut self, dir: Vec2, color: Rgb) -> bool {
        self.set_at_dir_from(dir, self.center_point, color)
    }

    /// Sets the color of each [LED](Led) in the given direction from a given point.
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED in that direction, true otherwise.
    ///
    /// O(SEGMENTS)
    pub fn set_at_dir_from(&mut self, dir: Vec2, pos: Vec2, color: Rgb) -> bool {
        let intersecting_indices = self.raycast_for_indices(pos, dir);

        if intersecting_indices.is_empty() {
            return false;
        }

        for index in intersecting_indices {
            self.leds[index].color = color;
        }

        true
    }

    /* angle setters/getters */

    /// Returns A [Filter] containing each [LED](Led) whose direction relative to the center point forms a given radian angle.
    ///
    /// Equivalent to `at_dir(Vec2::new(angle.cos(), angle.sin()));`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// If no LEDs exist at the given direction, the Filter will be empty.
    ///
    /// Currently returns no more than 4 LEDs, may change in the future.
    ///
    /// O(SEGMENTS)
    pub fn at_angle(&self, angle: f32) -> Filter {
        let dir = Vec2::from_angle(angle);
        self.at_dir(dir)
    }

    /// Returns A [Filter] containing each [LED](Led) whose direction relative to a point forms a given radian angle.
    ///
    /// Equivalent to `at_dir_from(Vec2::new(angle.cos(), angle.sin()), pos);`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// If no LEDs exist at the given direction, the Filter will be empty.
    ///
    /// Currently returns no more than 4 LEDs, may change in the future.
    ///
    /// O(SEGMENTS)
    pub fn at_angle_from(&self, angle: f32, pos: Vec2) -> Filter {
        let dir = Vec2::from_angle(angle);
        self.at_dir_from(dir, pos)
    }

    /// Modulates the color of each [LED](Led) whose direction relative to the center point forms a given radian angle.
    ///
    /// Equivalent to `modulate_at_dir(Vec2::new(angle.cos(), angle.sin()), /*-snip-*/);`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED at that angle, true otherwise.
    ///
    /// O(SEGMENTS)
    ///
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    /// use core::f32::consts::PI;
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// sled.modulate_at_angle(PI / 4.0, |led| led.color * 2.0);
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_at_angle<F: Fn(&Led) -> Rgb>(&mut self, angle: f32, color_rule: F) -> bool {
        self.modulate_at_angle_from(angle, self.center_point, color_rule)
    }

    /// Modulates the color of each [LED](Led) whose direction relative to a point forms a given radian angle.
    ///
    /// Equivalent to `modulate_at_dir_from(Vec2::new(angle.cos(), angle.sin()), pos, /*-snip-*/);`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED at that angle, true otherwise.
    ///
    /// O(SEGMENTS)
    ///
    ///```rust
    ///# use spatial_led::{Sled, SledError, color::Rgb, Vec2};
    /// use core::f32::consts::PI;
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.yap")?;
    /// let angle = PI * 1.25;
    /// let from = Vec2::new(0.3, 0.2);
    /// sled.modulate_at_angle_from(angle, from, |led| led.color * 2.0);
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_at_angle_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        angle: f32,
        pos: Vec2,
        color_rule: F,
    ) -> bool {
        let dir = Vec2::from_angle(angle);
        self.modulate_at_dir_from(dir, pos, color_rule)
    }

    /// Sets the color of each [LED](Led) whose direction relative to the center point forms a given radian angle.
    /// Equivalent to `set_at_dir(Vec2::new(angle.cos(), angle.sin()), color);`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED at that angle, true otherwise.
    ///
    /// O(SEGMENTS)
    pub fn set_at_angle(&mut self, angle: f32, color: Rgb) -> bool {
        self.set_at_angle_from(angle, self.center_point, color)
    }

    /// Sets the color of each [LED](Led) whose direction relative to a point forms a given radian angle.
    /// Equivalent to `set_at_dir(Vec2::new(angle.cos(), angle.sin()), pos, color);`
    ///
    /// Calculated by performing a raycast against each line segment and finding the closest LED to the point of contact.
    ///
    /// Returns false if there is no LED at that angle, true otherwise.
    ///
    /// O(SEGMENTS)
    pub fn set_at_angle_from(&mut self, angle: f32, pos: Vec2, color: Rgb) -> bool {
        let dir = Vec2::from_angle(angle);
        self.set_at_dir_from(dir, pos, color)
    }
}
