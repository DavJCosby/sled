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
            .min_by(|l, r| l.distance().partial_cmp(&r.distance()).unwrap())
            .unwrap()
            .index();

        let index_of_furthest = leds
            .iter()
            .max_by(|l, r| l.distance().partial_cmp(&r.distance()).unwrap())
            .unwrap()
            .index();

        Ok(Sled {
            center_point: config.center_point,
            leds,
            num_leds,
            line_segments: config.line_segments,
            index_of_closest,
            index_of_furthest,
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

    pub fn read_positions(&self) -> Vec<Vec2> {
        self.leds.iter().map(|led| led.position()).collect()
    }

    pub fn center_point(&self) -> Vec2 {
        self.center_point
    }

    pub fn num_leds(&self) -> usize {
        self.num_leds
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
                let led = Led::new(default_color, pos, leds.len(), segment_index, *center_point);

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

    pub(crate) fn alpha_to_index(&self, segment_alpha: f32, segment_index: usize) -> usize {
        let segment = &self.line_segments[segment_index];
        let startpoint_index = self.line_segment_endpoint_indices[segment_index].0;
        let leds_in_segment = segment.num_leds() as f32;

        startpoint_index + (segment_alpha * leds_in_segment).floor() as usize
    }
}
