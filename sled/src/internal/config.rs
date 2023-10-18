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
