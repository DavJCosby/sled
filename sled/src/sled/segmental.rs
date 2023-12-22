use crate::{
    color::Rgb,
    error::SledError,
    led::Led,
    sled::{Set, Sled},
};
use std::{collections::HashSet, ops::Range};

/// Segment-based read and write methods.
impl Sled {
    pub fn get_segment(&self, segment_index: usize) -> Option<Set> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        let led_range = &self.leds[start..end];
        Some(led_range.into())
    }

    pub fn modulate_segment<F: Fn(&Led) -> Rgb>(
        &mut self,
        segment_index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if segment_index >= self.line_segment_endpoint_indices.len() {
            return Err(SledError {
                message: format!("Segment of index {} does not exist.", segment_index),
            });
        }

        let (start, end) = self.line_segment_endpoint_indices[segment_index];
        for led in &mut self.leds[start..end] {
            led.color = color_rule(led);
        }

        Ok(())
    }

    pub fn set_segment(&mut self, segment_index: usize, color: Rgb) -> Result<(), SledError> {
        if segment_index >= self.line_segment_endpoint_indices.len() {
            return Err(SledError {
                message: format!("No line segment of index {} exists.", segment_index),
            });
        }

        let (start, end) = self.line_segment_endpoint_indices[segment_index];
        self.set_range(start..end, color).unwrap();
        Ok(())
    }

    pub fn get_segments(&self, range: Range<usize>) -> Option<Set> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            None
        } else {
            let (start, _) = *self.line_segment_endpoint_indices.get(range.start)?;
            let (_, end) = *self.line_segment_endpoint_indices.get(range.end)?;
            let led_range = &self.leds[start..end];
            Some(led_range.into())
        }
    }

    pub fn modulate_segments<F: Fn(&Led) -> Rgb>(
        &mut self,
        range: Range<usize>,
        color_rule: F,
    ) -> Result<(), SledError> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            return Err(SledError {
                message: "Segment index range extends beyond the number of segments in the system.".to_string(),
            });
        }

        let (start, _) = self.line_segment_endpoint_indices[range.start];
        let (_, end) = self.line_segment_endpoint_indices[range.end];
        for led in &mut self.leds[start..end] {
            led.color = color_rule(led);
        }
        Ok(())
    }

    pub fn set_segments(&mut self, range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            return Err(SledError {
                message: "Segment index range extends beyond the number of segments in the system.".to_string(),
            });
        }

        let (start, _) = self.line_segment_endpoint_indices[range.start];
        let (_, end) = self.line_segment_endpoint_indices[range.end];
        for led in &mut self.leds[start..end] {
            led.color = color;
        }
        Ok(())
    }

    pub fn for_each_in_segment<F: FnMut(&mut Led, f32)>(
        &mut self,
        segment_index: usize,
        mut func: F,
    ) -> Result<(), SledError> {
        if segment_index >= self.line_segment_endpoint_indices.len() {
            return Err(SledError {
                message: format!("No line segment of index {} exists.", segment_index),
            });
        }

        let (start, end) = self.line_segment_endpoint_indices[segment_index];
        let num_leds_f32 = (end - start) as f32;

        for index in start..end {
            let alpha = (index - start) as f32 / num_leds_f32;
            func(&mut self.leds[index], alpha);
        }

        Ok(())
    }
}

/// Vertex-based read and write methods.
impl Sled {
    pub fn get_vertex(&self, vertex_index: usize) -> Option<&Led> {
        if vertex_index >= self.vertex_indices.len() {
            return None;
        }

        Some(&self.leds[vertex_index])
    }

    pub fn modulate_vertex<F: Fn(&Led) -> Rgb>(
        &mut self,
        vertex_index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if vertex_index >= self.vertex_indices.len() {
            return Err(SledError {
                message: format!("Vertex of index {} does not exist.", vertex_index),
            });
        }

        let led = &mut self.leds[vertex_index];
        led.color = color_rule(led);
        Ok(())
    }

    pub fn set_vertex(&mut self, vertex_index: usize, color: Rgb) -> Result<(), SledError> {
        if vertex_index >= self.vertex_indices.len() {
            return Err(SledError {
                message: format!("Vertex with index {} does not exist.", vertex_index),
            });
        }

        self.leds[self.vertex_indices[vertex_index]].color = color;
        Ok(())
    }

    pub fn get_vertices(&self) -> Set {
        let hs: HashSet<&Led> = self.vertex_indices.iter().map(|i| &self.leds[*i]).collect();
        hs.into()
    }

    pub fn modulate_vertices<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        for i in &self.vertex_indices {
            let led = &mut self.leds[*i];
            led.color = color_rule(led);
        }
    }

    pub fn set_vertices(&mut self, color: Rgb) {
        for i in &self.vertex_indices {
            let led = &mut self.leds[*i];
            led.color = color;
        }
    }

    pub fn for_each_vertex<F: FnMut(&mut Led)>(&mut self, mut f: F) {
        for i in &self.vertex_indices {
            f(&mut self.leds[*i])
        }
    }
}
