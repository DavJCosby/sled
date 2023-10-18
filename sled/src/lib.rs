mod internal;

pub mod color {
    pub use palette::rgb::Rgb;
    pub use palette::*;
}

pub use internal::error::SledError;

use glam::Vec2;
use internal::config::{Config, LineSegment};
use std::{ops::Range, usize};

use color::{Rgb, Srgb};

#[allow(dead_code)]
pub struct Sled {
    center_point: Vec2,
    leds: Vec<Rgb>,
    line_segments: Vec<LineSegment>,
    line_segment_endpoint_indices: Vec<(usize, usize)>,
    vertex_indices: Vec<usize>,
}

/// Construction, output, and basic sled info.
impl Sled {
    pub fn new(config_file_path: &str) -> Result<Self, SledError> {
        let config = Config::from_toml_file(config_file_path)?;
        let leds_per_strip = Sled::leds_per_strip(&config);

        Ok(Sled {
            leds: vec![Rgb::new(0.0, 0.0, 0.0); leds_per_strip.iter().sum()],
            line_segment_endpoint_indices: Sled::line_segment_endpoint_indices(leds_per_strip),
            vertex_indices: Sled::vertex_indices(&config),
            center_point: config.center_point,
            line_segments: config.line_segments,
        })
    }

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

    pub fn num_leds(&self) -> usize {
        self.leds.len()
    }

    pub fn num_segments(&self) -> usize {
        self.line_segments.len()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertex_indices.len()
    }

    fn leds_per_strip(config: &Config) -> Vec<usize> {
        config
            .line_segments
            .iter()
            .map(|line| line.num_leds())
            .collect()
    }

    fn vertex_indices(config: &Config) -> Vec<usize> {
        let mut vertex_indices = vec![];

        let mut last_end_point: Option<Vec2> = None;
        let mut last_index = 0;
        for line in &config.line_segments {
            if Some(line.start) != last_end_point {
                vertex_indices.push(last_index);
            }

            let num_leds = line.num_leds();
            vertex_indices.push(last_index + num_leds - 1);

            last_index += num_leds;
            last_end_point = Some(line.end);
        }

        vertex_indices
    }

    fn line_segment_endpoint_indices(leds_per_strip: Vec<usize>) -> Vec<(usize, usize)> {
        let mut line_segment_endpoint_indices = vec![];
        let mut last_index = 0;
        for num_leds in &leds_per_strip {
            line_segment_endpoint_indices.push((last_index, last_index + num_leds));
            last_index += num_leds;
        }

        line_segment_endpoint_indices
    }
}

// Index-based read and write methods.
impl Sled {
    pub fn get(&self, index: usize) -> Option<&Rgb> {
        self.leds.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Rgb> {
        self.leds.get_mut(index)
    }

    pub fn get_range(&self, index_range: Range<usize>) -> &[Rgb] {
        &self.leds[index_range]
    }

    pub fn get_range_mut(&mut self, index_range: Range<usize>) -> &mut [Rgb] {
        &mut self.leds[index_range]
    }

    pub fn set(&mut self, index: usize, color: Rgb) -> Result<(), SledError> {
        let led = self.get_mut(index);
        match led {
            Some(rgb) => *rgb = color,
            None => {
                return Err(SledError {
                    message: format!("LED at index {} does not exist.", index),
                })
            }
        }

        Ok(())
    }

    pub fn set_all(&mut self, color: Rgb) {
        for led_color in self.leds.iter_mut() {
            *led_color = color;
        }
    }

    pub fn set_range(&mut self, index_range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        for index in index_range {
            match self.set(index, color) {
                Ok(_) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

/// LineSegment-based read and write methods.
impl Sled {
    pub fn get_segment(&self, segment_index: usize) -> Option<&[Rgb]> {
        let queried_segment = self.line_segment_endpoint_indices.get(segment_index);
        match queried_segment {
            Some(indices) => {
                let first = indices.0;
                let last = indices.1;
                Some(self.get_range(first..last))
            }
            None => None,
        }
    }

    pub fn get_segment_mut(&mut self, segment_index: usize) -> Option<&mut [Rgb]> {
        let queried_segment = self.line_segment_endpoint_indices.get(segment_index);
        match queried_segment {
            Some(indices) => {
                let first = indices.0;
                let last = indices.1;
                Some(self.get_range_mut(first..last))
            }
            None => None,
        }
    }

    pub fn set_segment(&mut self, segment_index: usize, color: Rgb) -> Result<(), SledError> {
        let leds = self.get_segment_mut(segment_index);
        match leds {
            Some(leds) => {
                for led in leds {
                    *led = color;
                }
            }
            None => {
                return Err(SledError {
                    message: format!("No line segment of index {} exists.", segment_index),
                });
            }
        }

        Ok(())
    }
}

/// Vertex-based read and write methods.
impl Sled {
    pub fn get_vertex(&self, vertex_index: usize) -> Option<&Rgb> {
        let led_index = self.vertex_indices.get(vertex_index);
        match led_index {
            Some(i) => self.get(*i),
            None => None,
        }
    }

    pub fn get_vertex_mut(&mut self, vertex_index: usize) -> Option<&mut Rgb> {
        let led_index = self.vertex_indices.get(vertex_index);
        match led_index {
            Some(i) => self.get_mut(*i),
            None => None,
        }
    }
}
