use std::fs;

use crate::SLEDError;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub center_point: Vec2,
    #[serde(rename = "line_segment")]
    pub line_segments: Vec<LineSegment>,
}

impl Config {
    pub fn from_toml_file(path: &str) -> Result<Self, SLEDError> {
        let file_contents = fs::read_to_string(path).map_err(SLEDError::from_error)?;
        let config = toml::from_str(&file_contents).map_err(SLEDError::from_error)?;
        // not totally proud of these map_err shenanigans, see if we can figure out how to make the
        // implicit .into() magic that normally happens with the ? operator compatible here.
        Ok(config)
    }

    // not 100% on the idea of this being a responsibility of config, may change later.
    pub fn num_leds(&self) -> usize {
        self.line_segments
            .iter()
            .map(|line| line.length() * line.density)
            .sum::<f32>() as usize
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub density: f32,
}

impl LineSegment {
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }
}
