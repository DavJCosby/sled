use std::ops::Range;

use crate::{
    color,
    color::{Rgb, Srgb},
    config::{Config, LineSegment},
    error::SledError,
    led::Led,
    sled::Sled,
    Vec2,
};

/// Construction, output, and basic sled info.
impl Sled {
    /// Constructs a new Sled struct given the path to a config toml file.
    /// This is an expensive operation as many values are pre-calculated
    /// on construction (i.e, distances/angles from each LED to the center).
    ///
    /// Example .toml file:
    /// ```ignore
    /// center_point = [0.0, 0.5]
    /// density = 30.0
    ///
    /// [[line_segment]]
    /// start = [-2.0, 0.0]
    /// end = [0.5, -1.0]
    ///
    /// [[line_segment]]
    /// start = [0.5, -1.0]
    /// end = [3.5, 0.0]
    ///
    /// [[line_segment]]
    /// start = [3.5, 0.0]
    /// end = [2, 2]
    ///
    /// [[line_segment]]
    /// start = [2.0, 2]
    /// end = [-2.0, 2]
    ///
    /// [[line_segment]]
    /// start = [-2.0, 2]
    /// end = [-2.0, 0.0]
    /// ```
    /// * `center_point` is a static reference point you can use to speed up draw calls.
    /// At initialization, directions, distances, etc relative to this point are pre-calculated for each Led.
    ///
    /// * `density` represents how many LED's per unit we can expect for the line segments below. If one or more
    /// LED has a different density for whatever reason, you can override this default for each line_segment.
    ///
    /// * Add as many `[[line_segment]]` tags as you need to represent your scene.
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
        let num_leds = leds.len();
        let index_of_closest = leds
            .iter()
            .min_by(|l, r| l.distance_squared().partial_cmp(&&r.distance_squared()).unwrap())
            .unwrap()
            .index() as usize;

        let index_of_furthest = leds
            .iter()
            .max_by(|l, r| l.distance_squared().partial_cmp(&r.distance_squared()).unwrap())
            .unwrap()
            .index() as usize;

        let domain = Sled::calc_domain(&leds);

        Ok(Sled {
            center_point: config.center_point,
            leds,
            num_leds,
            line_segments: config.line_segments,
            index_of_closest,
            index_of_furthest,
            domain,
            // utility lookup tables
            line_segment_endpoint_indices,
            vertex_indices,
        })
    }

    /// Returns a copy of the system's LEDs, stored in a vector.
    /// ```rust
    ///# use sled::{Sled};
    ///# let sled = Sled::new("./examples/resources/config.toml").unwrap();
    /// for led in sled.read() {
    ///     println!("Segment {}, Index {}: {:?}",
    ///         led.segment(), led.index(), led.color
    ///     );
    /// }
    /// ```
    pub fn read(&self) -> Vec<Led> {
        self.leds.clone()
    }

    /// Returns the colors of each LED in the system, stored in a vector.
    /// ```rust
    ///# use sled::{Sled, color::Rgb};
    ///# let sled = Sled::new("./examples/resources/config.toml").unwrap();
    /// // 32 bits/channel by default
    /// let colors: Vec<Rgb> = sled.read_colors();
    /// // coerce to 8 bits/channel
    /// let colors_u8: Vec<Rgb<_, u8>> = sled.read_colors();
    /// ```
    pub fn read_colors<T>(&self) -> Vec<Srgb<T>>
    where
        f32: color::stimulus::IntoStimulus<T>,
    {
        self.leds
            .iter()
            .map(|led| led.color.into_format())
            .collect()
    }

    /// Returns the positions of each LED in the system, stored in a vector.
    pub fn read_positions(&self) -> Vec<Vec2> {
        self.leds.iter().map(|led| led.position()).collect()
    }

    /// Returns the positions and colors of each LED in the system, stored in a vector of `(Rgb, Vec2)`.
    /// Supports color coercion just like [Sled::read_colors()](read_colors())
    pub fn read_colors_and_positions<T>(&self) -> Vec<(Srgb<T>, Vec2)>
    where
        f32: color::stimulus::IntoStimulus<T>,
    {
        self.leds
            .iter()
            .map(|led| (led.color.into_format(), led.position()))
            .collect()
    }

    /// Returns the static reference point declared in the config file.
    pub fn center_point(&self) -> Vec2 {
        self.center_point
    }

    /// Returns the total number of LEDs in the system.
    pub fn num_leds(&self) -> usize {
        self.num_leds
    }

    /// Returns the total number of line segments in the system.
    pub fn num_segments(&self) -> usize {
        self.line_segments.len()
    }

    /// Returns the total number of vertices in the system.
    ///
    /// Touching endpoints are merged into one vertex, meaning that a
    /// configuration of two line segments that meet at one point to form
    /// a corner would have three vertices, rather than four.
    pub fn num_vertices(&self) -> usize {
        self.vertex_indices.len()
    }

    /// Returns a bounding box around the LEDs where the minimum x and y
    /// position is [Range::start], maximum x and y is [Range::end].
    pub fn domain(&self) -> Range<Vec2> {
        self.domain.clone()
    }

    fn leds_per_segment(config: &Config) -> Vec<usize> {
        config
            .line_segments
            .iter()
            .map(|line| line.num_leds())
            .collect()
    }

    fn build_led_list(
        leds_per_segment: &[usize],
        line_segments: &[LineSegment],
        center_point: &Vec2,
    ) -> Vec<Led> {
        let mut leds = vec![];
        let default_color = Rgb::new(0.0, 0.0, 0.0);

        for (segment_index, segment_size) in leds_per_segment.iter().enumerate() {
            for i in 0..*segment_size {
                let segment = &line_segments[segment_index];
                let alpha = (i + 1) as f32 / *segment_size as f32;
                let pos = segment.start.lerp(segment.end, alpha);
                let led = Led::new(
                    default_color,
                    pos,
                    leds.len() as u16,
                    segment_index as u8,
                    *center_point,
                );

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

        let start = config.line_segments[0].start;
        let end = config.line_segments[config.line_segments.len() - 1].end;

        if start != end {
            vertex_indices.push(0);
        }

        let mut last_end_point: Vec2 = start;
        let mut last_index = 0;
        for line in &config.line_segments {
            if line.start != last_end_point {
                vertex_indices.push(last_index);
            }

            let num_leds = line.num_leds();
            vertex_indices.push(last_index + num_leds - 1);

            last_index += num_leds;
            last_end_point = line.end;
        }

        vertex_indices
    }

    fn calc_domain(leds: &Vec<Led>) -> Range<Vec2> {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;

        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for led in leds {
            let p = led.position();

            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);

            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Vec2::new(min_x, min_y)..Vec2::new(max_x, max_y)
    }

    pub(crate) fn alpha_to_index(&self, segment_alpha: f32, segment_index: usize) -> usize {
        let segment = &self.line_segments[segment_index];
        let startpoint_index = self.line_segment_endpoint_indices[segment_index].0 as usize;
        let leds_in_segment = segment.num_leds() as f32;

        (startpoint_index + (segment_alpha * leds_in_segment).floor() as usize) % self.num_leds
    }
}
