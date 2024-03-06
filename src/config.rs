use crate::error::SledError;
use glam::Vec2;
use serde::{Deserialize, Deserializer, Serialize};
use smallvec::SmallVec;
use std::fs;

static mut DEFAULT_DENSITY: f32 = 0.0;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub center_point: Vec2,
    #[serde(rename = "density")]
    #[serde(deserialize_with = "Config::set_default_density")]
    pub default_density: f32,
    #[serde(rename = "line_segment")]
    pub line_segments: Vec<LineSegment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    #[serde(default = "Config::default_density")]
    pub density: f32,
}

impl LineSegment {
    pub fn num_leds(&self) -> usize {
        (self.length() * self.density).round() as usize
    }

    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    pub fn intersects_line(&self, other_start: Vec2, other_end: Vec2) -> Option<f32> {
        let s1 = self.end - self.start;
        let s2 = other_end - other_start;
        let start_dif = self.start - other_start;

        let denom = s1.x * s2.y - s2.x * s1.y;

        // check if parallel
        if denom.abs() < f32::EPSILON {
            return None;
        }

        let inv_denom = 1.0 / denom;
        let s = (-s1.y * start_dif.x + s1.x * start_dif.y) * inv_denom;
        let t = (s2.x * start_dif.y - s2.y * start_dif.x) * inv_denom;

        if (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t) {
            // Some((self.start + s1 * t, t))
            Some(t)
        } else {
            None
        }
    }

    pub fn intersects_circle(&self, circle_center: Vec2, circle_radius: f32) -> SmallVec<[f32; 2]> {
        let v1 = self.end - self.start;
        let v2 = self.start - circle_center;

        let b = -2.0 * v1.dot(v2);
        let c = 2.0 * v1.length_squared();
        let mut return_values = smallvec::smallvec![];

        let mut d = b * b - 2.0 * c * (v2.length_squared() - circle_radius.powi(2));
        if d < 0.0 {
            return return_values;
        }

        d = d.sqrt();

        let t1 = (b - d) / c;
        let t2 = (b + d) / c;

        if (0.0..=1.0).contains(&t1) {
            return_values.push(t1);
        }
        if (0.0..=1.0).contains(&t2) {
            return_values.push(t2);
        }

        return_values
    }

    pub fn closest_to_point(&self, point: Vec2) -> (Vec2, f32) {
        let atob = self.end - self.start;
        let atop = point - self.start;
        let len_sq = atob.length_squared();
        let dot = atop.dot(atob);
        let t = (dot / len_sq).clamp(0.0, 1.0);

        (self.start + atob * t, t)
    }
}

impl Config {
    pub fn from_toml_file(path: &str) -> Result<Self, SledError> {
        let file_contents = fs::read_to_string(path).map_err(SledError::from_error)?;
        let config = toml::from_str(&file_contents).map_err(SledError::from_error)?;
        Ok(config)
    }

    fn set_default_density<'de, D>(des: D) -> Result<f32, D::Error>
    where
        D: Deserializer<'de>,
    {
        // I hate this solution, for the record
        let den = f32::deserialize(des);
        unsafe { DEFAULT_DENSITY = den.unwrap_or(0.0) };
        Ok(unsafe { DEFAULT_DENSITY })
    }

    fn default_density() -> f32 {
        unsafe { DEFAULT_DENSITY }
    }
}
