mod internal;

pub use internal::color;
pub use internal::error::SledError;
pub use internal::led::Led;

use glam::Vec2;
use internal::config::{Config, LineSegment};
use std::{ops::Range, usize};

use color::{Rgb, Srgb};

#[allow(dead_code)]
pub struct Sled {
    center_point: Vec2,
    leds: Vec<Led>,
    line_segments: Vec<LineSegment>,
    line_segment_endpoint_indices: Vec<(usize, usize)>,
    vertex_indices: Vec<usize>,
}

/// Construction, output, and basic sled info.
impl Sled {
    pub fn new(config_file_path: &str) -> Result<Self, SledError> {
        let config = Config::from_toml_file(config_file_path)?;
        let leds_per_segment = Sled::leds_per_segment(&config);
        let leds = Sled::build_led_list(&leds_per_segment, &config.line_segments);
        let line_segment_endpoint_indices = Sled::line_segment_endpoint_indices(&leds_per_segment);
        let vertex_indices = Sled::vertex_indices(&config);
        Ok(Sled {
            leds,
            line_segment_endpoint_indices,
            vertex_indices,
            center_point: config.center_point,
            line_segments: config.line_segments,
        })
    }

    pub fn read_colors<T>(&self) -> Vec<Srgb<T>>
    where
        f32: color::stimulus::IntoStimulus<T>,
    {
        let mut output = vec![];
        for led in &self.leds {
            output.push(led.color.into_format());
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

    fn leds_per_segment(config: &Config) -> Vec<usize> {
        config
            .line_segments
            .iter()
            .map(|line| line.num_leds())
            .collect()
    }

    fn build_led_list(leds_per_segment: &Vec<usize>, line_segments: &Vec<LineSegment>) -> Vec<Led> {
        let mut leds = vec![];
        for (segment_index, segment_size) in leds_per_segment.iter().enumerate() {
            for i in 0..*segment_size {
                let a = i as f32 / (segment_size - 1) as f32;
                let segment = &line_segments[segment_index];
                let pos = segment.start.lerp(segment.end, a);
                leds.push(Led::new(Rgb::new(0.0, 0.0, 0.0), pos, segment_index));
            }
        }
        leds
    }

    fn line_segment_endpoint_indices(leds_per_segment: &Vec<usize>) -> Vec<(usize, usize)> {
        let mut line_segment_endpoint_indices = vec![];
        let mut last_index = 0;
        for num_leds in leds_per_segment {
            line_segment_endpoint_indices.push((last_index, last_index + num_leds));
            last_index += num_leds;
        }

        line_segment_endpoint_indices
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
}

/// Index-based read and write methods.
impl Sled {
    pub fn get(&self, index: usize) -> Option<&Led> {
        self.leds.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Led> {
        self.leds.get_mut(index)
    }

    pub fn set(&mut self, index: usize, color: Rgb) -> Result<(), SledError> {
        let led = self.get_mut(index).ok_or(SledError {
            message: format!("LED at index {} does not exist.", index),
        })?;

        led.color = color;
        Ok(())
    }

    pub fn set_all(&mut self, color: Rgb) {
        for led in self.leds.iter_mut() {
            led.color = color;
        }
    }

    pub fn for_each<F: FnMut(&mut Led, usize)>(&mut self, mut func: F) {
        for (index, led) in self.leds.iter_mut().enumerate() {
            func(led, index);
        }
    }
}

/// Index range-based read and write methods
impl Sled {
    pub fn get_range(&self, index_range: Range<usize>) -> &[Led] {
        &self.leds[index_range]
    }

    pub fn get_range_mut(&mut self, index_range: Range<usize>) -> &mut [Led] {
        &mut self.leds[index_range]
    }

    pub fn set_range(&mut self, index_range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        for index in index_range {
            self.set(index, color)?
        }
        Ok(())
    }

    pub fn for_each_in_range<F: FnMut(&mut Led, usize)>(
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

/// Segment-based read and write methods.
impl Sled {
    pub fn get_segment(&self, segment_index: usize) -> Option<&[Led]> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        Some(self.get_range(start..end))
    }

    pub fn get_segment_mut(&mut self, segment_index: usize) -> Option<&mut [Led]> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        Some(self.get_range_mut(start..end))
    }

    pub fn set_segment(&mut self, segment_index: usize, color: Rgb) -> Result<(), SledError> {
        let leds = self.get_segment_mut(segment_index).ok_or(SledError {
            message: format!("No line segment of index {} exists.", segment_index),
        })?;

        for led in leds {
            led.color = color;
        }

        Ok(())
    }

    pub fn for_each_in_segment<F: FnMut(&mut Led, f32)>(
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
    pub fn get_vertex(&self, vertex_index: usize) -> Option<&Led> {
        let led_index = self.vertex_indices.get(vertex_index)?;
        self.get(*led_index)
    }

    pub fn get_vertex_mut(&mut self, vertex_index: usize) -> Option<&mut Led> {
        let led_index = self.vertex_indices.get(vertex_index)?;
        self.get_mut(*led_index)
    }

    pub fn get_vertices(&self) -> Vec<&Led> {
        let mut led_references: Vec<&Led> = vec![];
        for led_index in &self.vertex_indices {
            led_references.push(self.get(*led_index).unwrap());
        }

        led_references
    }

    pub fn get_vertices_mut(&mut self) -> Vec<&mut Led> {
        // a bit of an ugly solution, but it works. Take a vector of references to everything, then delete the ones you don't need.
        let mut everything = self.leds.iter_mut().collect::<Vec<&mut Led>>();
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

        led.color = color;
        Ok(())
    }

    pub fn set_vertices(&mut self, color: Rgb) {
        for i in self.vertex_indices.clone() {
            self.set(i, color).unwrap();
        }
    }

    pub fn for_each_vertex<F: FnMut(&mut Led)>(&mut self, mut f: F) {
        for i in &self.vertex_indices {
            f(&mut self.leds[*i])
        }
    }
}
