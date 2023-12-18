use crate::{color::Rgb, error::SledError, led::Led, sled::Sled};
use glam::Vec2;

/// directional read and write methods
impl Sled {
    fn raycast_for_index(&self, start: Vec2, dir: Vec2) -> Option<usize> {
        let dist = 100_000.0;
        let end = start + dir * dist;

        let mut intersection: Option<(f32, usize)> = None;
        for (index, segment) in self.line_segments.iter().enumerate() {
            if let Some(t) = segment.intersects_line(start, end) {
                intersection = Some((t, index));
                break;
            }
        }

        let (alpha, segment_index) = intersection?;
        return Some(self.alpha_to_index(alpha, segment_index));
    }

    /* direction setters/getters */

    pub fn get_at_dir(&self, dir: Vec2) -> Option<&Led> {
        self.get_at_dir_from(dir, self.center_point)
    }

    pub fn get_at_dir_from(&self, dir: Vec2, pos: Vec2) -> Option<&Led> {
        let index_of_closest = self.raycast_for_index(pos, dir)?;
        Some(self.get(index_of_closest)?)
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
        match self.raycast_for_index(pos, dir) {
            Some(index) => {
                let led = &mut self.leds[index];
                led.color = color_rule(&led);
                Ok(())
            }
            None => Err(SledError {
                message: format!("No LED in directon: {} from {}", dir, pos),
            }),
        }
    }

    pub fn set_at_dir(&mut self, dir: Vec2, color: Rgb) -> Result<(), SledError> {
        self.set_at_dir_from(dir, self.center_point, color)
    }

    pub fn set_at_dir_from(&mut self, dir: Vec2, pos: Vec2, color: Rgb) -> Result<(), SledError> {
        match self.raycast_for_index(pos, dir) {
            Some(index) => {
                self.leds[index].color = color;
                Ok(())
            }
            None => Err(SledError {
                message: format!("No LED in directon: {} from {}", dir, pos),
            }),
        }
    }

    /* angle setters/getters */

    pub fn get_at_angle(&self, angle: f32) -> Option<&Led> {
        let dir = Vec2::from_angle(angle);
        self.get_at_dir(dir)
    }

    pub fn get_at_angle_from(&self, angle: f32, center_point: Vec2) -> Option<&Led> {
        let dir = Vec2::from_angle(angle);
        self.get_at_dir_from(dir, center_point)
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
