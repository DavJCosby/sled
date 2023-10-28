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
        let led = self.get_mut(index).ok_or(SledError {
            message: format!("LED at index {} does not exist.", index),
        })?;

        *led = color;
        Ok(())
    }

    pub fn set_all(&mut self, color: Rgb) {
        for led_color in self.leds.iter_mut() {
            *led_color = color;
        }
    }

    pub fn for_each<F: FnMut(&mut Rgb, usize)>(&mut self, mut func: F) {
        for (index, led) in self.leds.iter_mut().enumerate() {
            func(led, index);
        }
    }

    pub fn set_range(&mut self, index_range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        for index in index_range {
            self.set(index, color)?
        }
        Ok(())
    }

    pub fn for_each_in_range<F: FnMut(&mut Rgb, usize)>(
        &mut self,
        index_range: Range<usize>,
        mut func: F,
    ) {
        let lower_bound = index_range.start;
        let range = self.get_range_mut(index_range);
        for (index, led) in range.iter_mut().enumerate() {
            func(led, lower_bound + index);
        }
    }
}

/// LineSegment-based read and write methods.
impl Sled {
    pub fn get_segment(&self, segment_index: usize) -> Option<&[Rgb]> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        Some(self.get_range(start..end))
    }

    pub fn get_segment_mut(&mut self, segment_index: usize) -> Option<&mut [Rgb]> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        Some(self.get_range_mut(start..end))
    }

    pub fn set_segment(&mut self, segment_index: usize, color: Rgb) -> Result<(), SledError> {
        let leds = self.get_segment_mut(segment_index).ok_or(SledError {
            message: format!("No line segment of index {} exists.", segment_index),
        })?;

        for led in leds {
            *led = color;
        }

        Ok(())
    }

    pub fn for_each_in_segment<F: FnMut(&mut Rgb, f32)>(
        &mut self,
        segment_index: usize,
        mut func: F,
    ) -> Result<(), SledError> {
        let segment = self.get_segment_mut(segment_index).ok_or(SledError {
            message: format!("No line segment of index {} exists.", segment_index),
        })?;

        let num_leds_f32 = segment.len() as f32;
        for (index, led) in segment.iter_mut().enumerate() {
            let alpha = index as f32 / num_leds_f32;
            func(led, alpha);
        }

        Ok(())
    }
}

/// Vertex-based read and write methods.
impl Sled {
    pub fn get_vertex(&self, vertex_index: usize) -> Option<&Rgb> {
        let led_index = self.vertex_indices.get(vertex_index)?;
        self.get(*led_index)
    }

    pub fn get_vertex_mut(&mut self, vertex_index: usize) -> Option<&mut Rgb> {
        let led_index = self.vertex_indices.get(vertex_index)?;
        self.get_mut(*led_index)
    }

    pub fn get_vertices(&self) -> Vec<&Rgb> {
        let mut led_references: Vec<&Rgb> = vec![];
        for led_index in &self.vertex_indices {
            led_references.push(self.get(*led_index).unwrap());
        }

        led_references
    }

    pub fn get_vertices_mut(&mut self) -> Vec<&mut Rgb> {
        // a bit of an ugly solution, but it works. Take a vector of references to everything, then delete the ones you don't need.
        let mut everything = self.leds.iter_mut().collect::<Vec<&mut Rgb>>();
        let mut vertices = vec![];
        for i in self.vertex_indices.iter().rev() {
            vertices.push(everything.swap_remove(*i));
        }
        vertices.reverse();
        vertices
    }

    pub fn set_vertex(&mut self, vertex_index: usize, color: Rgb) -> Result<(), SledError> {
        let led = self.get_vertex_mut(vertex_index).ok_or(SledError {
            message: format!("Vertex with index {} does not exist.", vertex_index),
        })?;

        *led = color;
        Ok(())
    }

    pub fn set_vertices(&mut self, color: Rgb) {
        for i in self.vertex_indices.clone() {
            self.set(i, color).unwrap();
        }
    }

    pub fn for_each_vertex<F: FnMut(&mut Rgb)>(&mut self, mut f: F) {
        for i in &self.vertex_indices {
            f(&mut self.leds[*i])
        }
    }
}
