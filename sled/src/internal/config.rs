use crate::SledError;
use glam::Vec2;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;

static mut DEFAULT_DENSITY: f32 = 0.0;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub center_point: Vec2,
    #[serde(rename = "density")]
    #[serde(deserialize_with = "Config::set_default_density")]
    pub default_density: f32,
    #[serde(rename = "line_segment")]
    pub line_segments: Vec<LineSegment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    #[serde(default = "Config::get_default_density")]
    pub density: f32,
}

impl LineSegment {
    pub fn num_leds(&self) -> usize {
        return (self.length() * self.density).round() as usize;
    }

    pub fn intersects(&self, other_start: Vec2, other_end: Vec2) -> Option<Vec2> {
        let s1_x = self.end.x - self.start.x;
        let s1_y = self.end.y - self.start.y;
        let s2_x = other_end.x - other_start.x;
        let s2_y = other_end.y - other_start.y;

        let denom = 1.0 / (-s2_x * s1_y + s1_x * s2_y);

        let s = (-s1_y * (self.start.x - other_start.x) + s1_x * (self.start.y - other_start.y))
            * denom;
        let t =
            (s2_x * (self.start.y - other_start.y) - s2_y * (self.start.x - other_start.x)) * denom;

        if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
            // collision detected
            Some(Vec2::new(
                self.start.x + (t * s1_x),
                self.start.y + (t * s1_y),
            ))
        } else {
            None
        }
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

    fn get_default_density() -> f32 {
        return unsafe { DEFAULT_DENSITY };
    }
}

impl LineSegment {
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }
}
