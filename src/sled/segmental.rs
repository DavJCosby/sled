use crate::{
    color::Rgb,
    error::SledError,
    led::Led,
    sled::{Filter, Sled},
};
use std::{collections::HashSet, ops::Range};

/// # Segment-based read and write methods.
impl Sled {
    /// Returns the set of all [LEDs](Led) assigned to the line segment with index `segment_index`.
    ///
    /// O(LEDS_IN_SEGMENT)
    pub fn segment(&self, segment_index: usize) -> Option<Filter> {
        let (start, end) = *self.line_segment_endpoint_indices.get(segment_index)?;
        let led_range = &self.leds[start..end];
        Some(led_range.into())
    }
    /// Modulates the color of each [LED](Led) assigned to the line segment with index `segment_index` given a color rule function. Returns an [error](SledError) if there is no line segment with the given index.
    /// O(LEDS_IN_SEGMENT)
    ///
    ///```rust
    ///# use sled::{Sled, SledError};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// sled.modulate_segment(1, |led| led.color * 2.0)?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_segment<F: Fn(&Led) -> Rgb>(
        &mut self,
        segment_index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if segment_index >= self.line_segment_endpoint_indices.len() {
            return SledError::new(format!(
                "Segment of index {} does not exist.",
                segment_index
            ))
            .as_err();
        }

        let (start, end) = self.line_segment_endpoint_indices[segment_index];
        for led in &mut self.leds[start..end] {
            led.color = color_rule(led);
        }

        Ok(())
    }

    /// Sets the color of each [LED](Led) assigned to the line segment with index `segment_index`. Returns an [error](SledError) if there is no line segment with the given index.
    /// O(LEDS_IN_SEGMENT)
    ///
    pub fn set_segment(&mut self, segment_index: usize, color: Rgb) -> Result<(), SledError> {
        if segment_index >= self.line_segment_endpoint_indices.len() {
            return SledError::new(format!(
                "No line segment of index {} exists.",
                segment_index
            ))
            .as_err();
        }

        let (start, end) = self.line_segment_endpoint_indices[segment_index];
        self.set_range(start..end, color).unwrap();
        Ok(())
    }

    /// Returns the set of all [LEDs](Led) assigned to the line segments whose indices are within the given range.
    /// 
    /// If the range exceeds the number of segments in the system, returns None.
    ///
    /// O(LEDS_IN_SEGMENTS)
    ///
    /// ```rust
    ///# use sled::{Sled, SledError, Filter, color::Rgb};
    ///# fn main() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// let first_three_walls: Filter = sled.segments(0..3).unwrap();
    /// sled.set_filter(&first_three_walls, Rgb::new(1.0, 1.0, 1.0));
    ///# Ok(())
    ///# }
    /// ```
    pub fn segments(&self, range: Range<usize>) -> Option<Filter> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            None
        } else {
            let (start, _) = *self.line_segment_endpoint_indices.get(range.start)?;
            let (_, end) = *self.line_segment_endpoint_indices.get(range.end)?;
            let led_range = &self.leds[start..end];
            Some(led_range.into())
        }
    }

    /// Modulates the color of each [LED](Led) assigned to the line segments whose indices are within the given range.
    /// Returns an [error](SledError) if the range exceeds the number of line segments in the system.
    ///
    /// O(LEDS_IN_SEGMENTS)
    ///
    /// ```rust
    ///# use sled::{Sled, SledError, color::Rgb};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// sled.modulate_segments(2..4, |led| led.color * Rgb::new(1.0, 0.0, 0.0))?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_segments<F: Fn(&Led) -> Rgb>(
        &mut self,
        range: Range<usize>,
        color_rule: F,
    ) -> Result<(), SledError> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            return SledError::new(
                "Segment index range extends beyond the number of segments in the system."
                    .to_string(),
            )
            .as_err();
        }

        let (start, _) = self.line_segment_endpoint_indices[range.start];
        let (_, end) = self.line_segment_endpoint_indices[range.end];
        for led in &mut self.leds[start..end] {
            led.color = color_rule(led);
        }
        Ok(())
    }

    /// Sets the color of each [LED](Led) assigned to the line segments whose indices are within the given range.
    /// Returns an [error](SledError) if the range exceeds the number of line segments in the system.
    /// O(LEDS_IN_SEGMENTS)
    pub fn set_segments(&mut self, range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        if range.start >= self.line_segment_endpoint_indices.len() {
            return SledError::new(
                "Segment index range extends beyond the number of segments in the system."
                    .to_string(),
            )
            .as_err();
        }

        let (start, _) = self.line_segment_endpoint_indices[range.start];
        let (_, end) = self.line_segment_endpoint_indices[range.end];
        for led in &mut self.leds[start..end] {
            led.color = color;
        }
        Ok(())
    }

    /// For-each method granting mutable access to each [LED](Led) assigned to the line segment with index `segment_index`.
    /// Also passes an "alpha" value into the closure, representing how far along the line segment you are. 0 = first LED in segement, 1 = last.
    ///
    /// Returns an [error](SledError) if the no segment of given index exists.
    /// 
    /// O(LEDS_IN_SEGMENT)
    ///
    /// ```rust
    ///# use sled::{Sled, color::Rgb};
    ///# let mut sled = Sled::new("./examples/resources/config.toml").unwrap();
    /// sled.for_each_in_segment(2, |led, alpha| {
    ///     led.color = Rgb::new(alpha, alpha, alpha);
    /// });
    /// ```
    /// ![segment alpha example](https://raw.githubusercontent.com/DavJCosby/sled/master/resources/segment_alpha.png)
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

/// # Vertex-based read and write methods.
impl Sled {
    /// Returns the [LED](Led) that represents the vertex the given index, if it exists.
    /// Vertices are distinct from line segement endpoints in that line segments with touching endpoints will share a vertex.
    ///
    /// O(1)
    ///
    pub fn vertex(&self, vertex_index: usize) -> Option<&Led> {
        if vertex_index >= self.vertex_indices.len() {
            return None;
        }

        Some(&self.leds[vertex_index])
    }
    /// Modulates the color of the [LED](Led) that represents the vertex the given index, if it exists. Returns an [error](SledError) if not.
    /// Vertices are distinct from line segement endpoints in that line segments with touching endpoints will share a vertex.
    ///
    /// Returns an [error](SledError) if no vertex of given index exists.
    /// 
    /// O(1)
    ///
    /// ```rust
    ///# use sled::{Sled, SledError, color::Rgb};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// // make the given vertex 25% brighter
    /// sled.modulate_vertex(3, |led| led.color * 1.25)?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_vertex<F: Fn(&Led) -> Rgb>(
        &mut self,
        vertex_index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if vertex_index >= self.vertex_indices.len() {
            return SledError::new(format!("Vertex of index {} does not exist.", vertex_index))
                .as_err();
        }

        let led = &mut self.leds[vertex_index];
        led.color = color_rule(led);
        Ok(())
    }

    /// Sets the color of the [LED](Led) that represents the vertex the given index, if it exists. Returns an [error](SledError) if not.
    /// Vertices are distinct from line segement endpoints in that line segments with touching endpoints will share a vertex.
    ///
    /// Returns an [error](SledError) if no vertex of given index exists.
    /// 
    /// O(1)
    ///
    pub fn set_vertex(&mut self, vertex_index: usize, color: Rgb) -> Result<(), SledError> {
        if vertex_index >= self.vertex_indices.len() {
            return SledError::new(format!(
                "Vertex with index {} does not exist.",
                vertex_index
            ))
            .as_err();
        }

        self.leds[self.vertex_indices[vertex_index]].color = color;
        Ok(())
    }

    /// Returns a [Filter] containing all vertices in the system.
    pub fn vertices(&self) -> Filter {
        let hs: HashSet<u16> = self.vertex_indices.iter().map(|i| *i as u16).collect();
        hs.into()
    }

    /// Modulates the color of each [LED](Led) that represents a vertex in the system.
    /// Vertices are distinct from line segement endpoints in that line segments with touching endpoints will share a vertex.
    ///
    /// O(VERTICES)
    ///
    /// ```rust
    ///# use sled::{Sled, SledError};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// // make each vertex 25% brighter
    /// sled.modulate_vertices(|led| led.color * 1.25);
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_vertices<F: Fn(&Led) -> Rgb>(&mut self, color_rule: F) {
        for i in &self.vertex_indices {
            let led = &mut self.leds[*i];
            led.color = color_rule(led);
        }
    }

    /// Sets the color of each [LED](Led) that represents a vertex in the system.
    ///
    /// O(VERTICES)
    pub fn set_vertices(&mut self, color: Rgb) {
        for i in &self.vertex_indices {
            let led = &mut self.leds[*i];
            led.color = color;
        }
    }

    /// For each method that grants mutable access to each [LED](Led) that represents a vertex in the system.
    ///
    /// O(VERTICES)
    pub fn for_each_vertex<F: FnMut(&mut Led)>(&mut self, mut f: F) {
        for i in &self.vertex_indices {
            f(&mut self.leds[*i])
        }
    }
}
