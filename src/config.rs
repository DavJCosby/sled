use crate::error::SledError;
use glam::Vec2;
use smallvec::SmallVec;
use std::{fs, str::Lines};

pub(crate) struct Config {
    pub center_point: Vec2,
    pub density: f32,
    pub line_segments: Vec<LineSegment>,
}

fn extract_center_and_density_from_lines(lines: &mut Lines) -> (Option<Vec2>, Option<f32>) {
    let mut center: Option<Vec2> = None;
    let mut density: Option<f32> = None;
    let mut segment_marker_found = false;

    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed.starts_with("--segments--") {
            segment_marker_found = true;
            break;
        } else if trimmed.starts_with("center:") {
            center = Some(get_center_from_line(line));
        } else if trimmed.starts_with("density:") {
            density = Some(get_density_from_line(line));
        }
    }

    if !segment_marker_found {
        panic!("Error parsing config file: no segment marker of form `--segments--` found.")
    }

    (center, density)
}

fn get_center_from_line(line: &str) -> Vec2 {
    let colon_pos = line.find(':').unwrap();
    parse_string_to_vec2(line[(colon_pos + 1)..line.len()].trim())
}

fn get_density_from_line(line: &str) -> f32 {
    let colon_pos = line.find(':').unwrap();
    line[(colon_pos + 1)..line.len()].trim().parse().unwrap()
}

fn parse_string_to_vec2(s: &str) -> Vec2 {
    if s.starts_with('(') & s.ends_with(')') {
        let sub: &str = &s[1..(s.len() - 1)];
        let nums: Vec<f32> = sub
            .split(',')
            .map(|s| {
                s.trim().parse().unwrap_or_else(|_| {
                    panic!("Error parsing config file: malformed Vec2: `{}`", s)
                })
            })
            .collect();
        if !nums.len() == 2 {
            panic!("Error parsing config file: malformed Vec2: {}", s);
        }
        Vec2::new(nums[0], nums[1])
    } else {
        panic!("Error parsing config file: malformed Vec2: `{}`", s);
    }
}

fn lines_to_string(lines: &mut Lines) -> String {
    let mut composite = String::from("");

    for line in lines.by_ref() {
        composite += line.trim();
    }

    composite
}

fn extract_segments_from_string(s: &str) -> Vec<LineSegment> {
    let connected: Vec<&str> = s.split("|").collect();
    let mut segments: Vec<LineSegment> = vec![];
    for sequence in connected {
        let vertex_strings: Vec<&str> = sequence.split("-->").map(|s| s.trim()).collect();
        let mut last_vertex: Option<Vec2> = None;
        for vertex_string in vertex_strings {
            let vertex = parse_string_to_vec2(vertex_string);
            if let Some(lv) = last_vertex {
                segments.push(LineSegment {
                    start: lv,
                    end: vertex,
                });
            }
            last_vertex = Some(vertex);
        }
    }

    segments
}

impl Config {
    pub fn from_string(string: String) -> Result<Self, SledError> {
        let mut lines = string.lines();

        let (center, density) = extract_center_and_density_from_lines(&mut lines);

        if center.is_none() {
            return Err(SledError::new(
                "Error parsing config file: no center point descriptor found.".to_string(),
            ));
        }
        if density.is_none() {
            return Err(SledError::new(
                "Error parsing config file: no density descriptor found.".to_string(),
            ));
        }

        let back_to_str = lines_to_string(&mut lines);
        let line_segments = extract_segments_from_string(&back_to_str);

        Ok(Config {
            density: density.unwrap(),
            center_point: center.unwrap(),
            line_segments,
        })
    }

    pub fn from_toml_file(path: &str) -> Result<Self, SledError> {
        let as_string = fs::read_to_string(path).map_err(SledError::from_error)?;
        Config::from_string(as_string)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

impl LineSegment {
    pub fn num_leds(&self, density: f32) -> usize {
        (self.length() * density).round() as usize
    }

    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    pub fn intersects_line(&self, other_start: Vec2, other_end: Vec2) -> Option<f32> {
        let s1 = self.end - self.start;
        let s2 = other_end - other_start;
        let start_dif = self.start - other_start;

        let denom = s1.x * s2.y - s2.x * s1.y;

        // check if parallel
        if denom.abs() < f32::EPSILON {
            return None;
        }

        let inv_denom = 1.0 / denom;
        let s = (-s1.y * start_dif.x + s1.x * start_dif.y) * inv_denom;
        let t = (s2.x * start_dif.y - s2.y * start_dif.x) * inv_denom;

        if (0.0..=1.0).contains(&s) && (0.0..=1.0).contains(&t) {
            // Some((self.start + s1 * t, t))
            Some(t)
        } else {
            None
        }
    }

    pub fn intersects_circle(&self, circle_center: Vec2, circle_radius: f32) -> SmallVec<[f32; 2]> {
        let v1 = self.end - self.start;
        let v2 = self.start - circle_center;

        let b = -2.0 * v1.dot(v2);
        let c = 2.0 * v1.length_squared();
        let mut return_values = smallvec::smallvec![];

        let mut d = b * b - 2.0 * c * (v2.length_squared() - circle_radius.powi(2));
        if d < 0.0 {
            return return_values;
        }

        d = d.sqrt();

        let t1 = (b - d) / c;
        let t2 = (b + d) / c;

        if (0.0..=1.0).contains(&t1) {
            return_values.push(t1);
        }
        if (0.0..=1.0).contains(&t2) {
            return_values.push(t2);
        }

        return_values
    }

    pub fn closest_to_point(&self, point: Vec2) -> (Vec2, f32) {
        let atob = self.end - self.start;
        let atop = point - self.start;
        let len_sq = atob.length_squared();
        let dot = atop.dot(atob);
        let t = (dot / len_sq).clamp(0.0, 1.0);

        (self.start + atob * t, t)
    }
}
