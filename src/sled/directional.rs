use std::collections::HashSet;

use crate::{color::Rgb, error::SledError, led::Led, Filter, Sled};
use glam::Vec2;
use smallvec::SmallVec;

/// directional read and write methods
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

    pub fn at_dir(&self, dir: Vec2) -> Filter {
        self.at_dir_from(dir, self.center_point)
    }

    pub fn at_dir_from(&self, dir: Vec2, pos: Vec2) -> Filter {
        let intersecting_indices = self.raycast_for_indices(pos, dir);
        intersecting_indices
            .iter()
            .map(|i| *i as u16)
            .collect::<HashSet<u16>>()
            .into()
    }

    pub fn modulate_at_dir<F: Fn(&Led) -> Rgb>(
        &mut self,
        dir: Vec2,
        color_rule: F,
    ) -> Result<(), SledError> {
        self.modulate_at_dir_from(dir, self.center_point, color_rule)
    }

    pub fn modulate_at_dir_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        dir: Vec2,
        pos: Vec2,
        color_rule: F,
    ) -> Result<(), SledError> {
        let intersecting_indices = self.raycast_for_indices(pos, dir);

        if intersecting_indices.is_empty() {
            return SledError::new(format!("No LED in directon: {} from {}", dir, pos)).as_err();
        }

        for index in intersecting_indices {
            let led = &mut self.leds[index];
            led.color = color_rule(led);
        }
        Ok(())
    }

    pub fn set_at_dir(&mut self, dir: Vec2, color: Rgb) -> Result<(), SledError> {
        self.set_at_dir_from(dir, self.center_point, color)
    }

    pub fn set_at_dir_from(&mut self, dir: Vec2, pos: Vec2, color: Rgb) -> Result<(), SledError> {
        let intersecting_indices = self.raycast_for_indices(pos, dir);

        if intersecting_indices.is_empty() {
            return SledError::new(format!("No LED in directon: {} from {}", dir, pos)).as_err();
        }

        for index in intersecting_indices {
            self.leds[index].color = color;
        }
        Ok(())
    }

    /* angle setters/getters */

    pub fn at_angle(&self, angle: f32) -> Filter {
        let dir = Vec2::from_angle(angle);
        self.at_dir(dir)
    }

    pub fn at_angle_from(&self, angle: f32, center_point: Vec2) -> Filter {
        let dir = Vec2::from_angle(angle);
        self.at_dir_from(dir, center_point)
    }

    pub fn modulate_at_angle<F: Fn(&Led) -> Rgb>(
        &mut self,
        angle: f32,
        color_rule: F,
    ) -> Result<(), SledError> {
        self.modulate_at_angle_from(angle, self.center_point, color_rule)
    }

    pub fn modulate_at_angle_from<F: Fn(&Led) -> Rgb>(
        &mut self,
        angle: f32,
        pos: Vec2,
        color_rule: F,
    ) -> Result<(), SledError> {
        let dir = Vec2::from_angle(angle);
        self.modulate_at_dir_from(dir, pos, color_rule)
    }

    pub fn set_at_angle(&mut self, angle: f32, color: Rgb) -> Result<(), SledError> {
        self.set_at_angle_from(angle, self.center_point, color)
    }

    pub fn set_at_angle_from(
        &mut self,
        angle: f32,
        pos: Vec2,
        color: Rgb,
    ) -> Result<(), SledError> {
        let dir = Vec2::from_angle(angle);
        self.set_at_dir_from(dir, pos, color)
    }
}
