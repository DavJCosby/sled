use std::{error::Error, fmt, usize};

mod internal;
use glam::Vec2;
use internal::config::{Config, LineSegment};

pub mod color {
    pub use palette::rgb::Rgb;
    pub use palette::*;
}

use color::{Rgb, Srgb};

#[allow(dead_code)]
pub struct Sled {
    center_point: Vec2,
    leds: Vec<Rgb>,
    line_segments: Vec<LineSegment>,
}

impl Sled {
    pub fn new(config_file_path: &str) -> Result<Self, SledError> {
        let config = Config::from_toml_file(config_file_path)?;
        let leds_per_strip = config
            .line_segments
            .iter()
            .map(|line| line.length() * line.density);

        let total_leds = leds_per_strip.sum::<f32>() as usize;

        let leds = vec![Rgb::new(0.0, 0.0, 0.0); total_leds];
        // 4. create various utility maps to help us out later when we need to track down the specific leds.

        // 5. construct
        Ok(Sled {
            center_point: config.center_point,
            line_segments: config.line_segments,
            leds,
        })
    }
}

impl Sled {
    pub fn read<T>(&self) -> Vec<Srgb<T>>
    where
        f32: color::stimulus::IntoStimulus<T>,
    {
        let mut output = vec![];
        for color in &self.leds {
            output.push(color.into_format());
        }
        output
    }

    pub fn get_color(&self, index: usize) -> Option<&Rgb> {
        self.leds.get(index)
    }

    pub fn get_color_mut(&mut self, index: usize) -> Option<&mut Rgb> {
        self.leds.get_mut(index)
    }

    pub fn set<T: Into<usize>>(&mut self, index: T, color: Rgb) -> Result<(), SledError> {
        let index = index.into();
        let led = self.get_color_mut(index);
        match led {
            Some(rgb) => *rgb = color,
            None => {
                return Err(SledError::new(
                    format!("LED at index {} does not exist.", index).as_str(),
                ))
            }
        }

        Ok(())
    }

    pub fn set_all(&mut self, color: Rgb) {
        for led_color in self.leds.iter_mut() {
            *led_color = color;
        }
    }

    pub fn set_range(
        &mut self,
        range: std::ops::Range<usize>,
        color: Rgb,
    ) -> Result<(), SledError> {
        for index in range {
            match self.set(index, color) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct SledError {
    message: String,
}

impl SledError {
    pub fn new(message: &str) -> Self {
        SledError {
            message: message.to_string(),
        }
    }

    pub fn from_error(e: impl Error) -> Self {
        SledError {
            message: e.to_string(),
        }
    }
}

impl fmt::Display for SledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SledError {} // seems we can't have both. Might not be the best design; reconsider.
