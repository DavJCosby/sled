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
    // utility lookup tables
    line_segment_endpoint_indices: Vec<(usize, usize)>,
    vertex_indices: Vec<usize>,
}

/// Construction, output, and basic sled info.
impl Sled {
    pub fn new(config_file_path: &str) -> Result<Self, SledError> {
        let config = Config::from_toml_file(config_file_path)?;
        let leds_per_segment = Sled::leds_per_segment(&config);
        let leds = Sled::build_led_list(
            &leds_per_segment,
            &config.line_segments,
            &config.center_point,
        );
        let line_segment_endpoint_indices = Sled::line_segment_endpoint_indices(&leds_per_segment);
        let vertex_indices = Sled::vertex_indices(&config);
        println!("{}", config.center_point);
        Ok(Sled {
            center_point: config.center_point,
            leds,
            line_segments: config.line_segments,
            // utility lookup tables
            line_segment_endpoint_indices,
            vertex_indices,
        })
    }

    pub fn read(&self) -> Vec<Led> {
        self.leds.clone()
    }

    pub fn read_colors<T>(&self) -> Vec<Srgb<T>>
    where
        f32: color::stimulus::IntoStimulus<T>,
    {
        self.leds
            .iter()
            .map(|led| led.color.into_format())
            .collect()
    }

    pub fn center_point(&self) -> Vec2 {
        self.center_point
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

    fn build_led_list(
        leds_per_segment: &Vec<usize>,
        line_segments: &Vec<LineSegment>,
        center_point: &Vec2,
    ) -> Vec<Led> {
        let mut leds = vec![];
        let default_color = Rgb::new(0.0, 0.0, 0.0);

        for (segment_index, segment_size) in leds_per_segment.iter().enumerate() {
            for i in 0..*segment_size {
                let segment = &line_segments[segment_index];
                let alpha = i as f32 / (segment_size - 1) as f32;

                let pos = segment.start.lerp(segment.end, alpha);
                let dir = (pos - *center_point).normalize();

                let led = Led::new(default_color, pos, dir, leds.len(), segment_index);
                leds.push(led);
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

    pub fn for_each<F: FnMut(&mut Led)>(&mut self, mut func: F) {
        for led in self.leds.iter_mut() {
            func(led);
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

    pub fn for_each_in_range<F: FnMut(&mut Led)>(
        &mut self,
        index_range: Range<usize>,
        mut func: F,
    ) {
        let range = self.get_range_mut(index_range);
        for led in range.iter_mut() {
            func(led);
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
        let lower_bound = segment[0].index();
        for led in segment.iter_mut() {
            let alpha = (led.index() - lower_bound) as f32 / num_leds_f32;
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

fn reverse_lerp(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    if a.x != b.x {
        (c.x - a.x) / (b.x - a.x)
    } else {
        (c.y - a.y) / (b.y - a.y)
    }
}

/// directional read and write methods
impl Sled {
    fn raycast_for_index(&self, origin: Vec2, dir: Vec2) -> Option<usize> {
        let dist = 100_000.0;
        let ray_end = origin + dir * dist;

        let mut led_count = 0;
        let mut intersection: Option<(Vec2, usize)> = None;
        for (index, strip) in self.line_segments.iter().enumerate() {
            if let Some(point) = strip.intersects(origin, ray_end) {
                intersection = Some((point, index));
                break;
            }
            led_count += strip.num_leds();
        }

        let intersection = intersection?;
        let segment = &self.line_segments[intersection.1];
        let alpha = reverse_lerp(segment.start, segment.end, intersection.0);

        led_count += (alpha * segment.num_leds() as f32).round() as usize;
        if led_count > 0 {
            return Some(led_count - 1);
        } else {
            return Some(led_count);
        }
    }

    pub fn get_at_dir_from(&self, center_point: Vec2, dir: Vec2) -> Option<&Led> {
        let index_of_closest = self.raycast_for_index(center_point, dir)?;
        Some(self.get(index_of_closest)?)
    }

    pub fn get_at_angle_from(&self, center_point: Vec2, angle: f32) -> Option<&Led> {
        let dir = Vec2::from_angle(angle);
        self.get_at_dir_from(center_point, dir)
    }

    pub fn get_at_dir(&self, dir: Vec2) -> Option<&Led> {
        self.get_at_dir_from(self.center_point, dir)
    }

    pub fn get_at_angle(&self, angle: f32) -> Option<&Led> {
        let dir = Vec2::from_angle(angle);
        self.get_at_dir(dir)
    }

    pub fn get_at_dir_from_mut(&mut self, center_point: Vec2, dir: Vec2) -> Option<&mut Led> {
        let index_of_closest = self.raycast_for_index(center_point, dir)?;
        Some(self.get_mut(index_of_closest)?)
    }

    pub fn get_at_angle_from_mut(&mut self, center_point: Vec2, angle: f32) -> Option<&mut Led> {
        let dir = Vec2::from_angle(angle);
        self.get_at_dir_from_mut(center_point, dir)
    }

    pub fn get_at_dir_mut(&mut self, dir: Vec2) -> Option<&mut Led> {
        let index_of_closest = self.raycast_for_index(self.center_point, dir)?;
        self.get_mut(index_of_closest)
    }

    pub fn get_at_angle_mut(&mut self, angle: f32) -> Option<&mut Led> {
        self.get_at_angle_from_mut(self.center_point, angle)
    }

    pub fn set_at_dir(&mut self, dir: Vec2, color: Rgb) -> Result<(), SledError> {
        let led = self.get_at_dir_mut(dir).ok_or(SledError {
            message: format!("No LED in directon: {}", dir),
        })?;

        led.color = color;
        Ok(())
    }

    pub fn set_at_angle(&mut self, angle: f32, color: Rgb) -> Result<(), SledError> {
        let led = self.get_at_angle_mut(angle).ok_or(SledError {
            message: format!("No LED at angle: {}", angle),
        })?;

        led.color = color;
        Ok(())
    }
}
