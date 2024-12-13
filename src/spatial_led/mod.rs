use core::ops::Range;

use alloc::vec::Vec;

use crate::{color::ColorType, config::LineSegment, led::Led, Vec2};

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// A struct representing the layout of some LED configuration in 2D space, composed of line segments.
///
/// Sled structs are [constructed](Sled::new) from a .toml file that describe this layout.
/// Upon construction, key information like the indices of vertices or the angle from each led from the center_point is precalculated and cached for faster access later.
pub struct Sled<Color: ColorType> {
    center_point: Vec2,
    leds: Vec<Led<Color>>,
    num_leds: usize,
    density: f32,
    line_segments: Vec<LineSegment>,
    // utility lookup tables
    line_segment_endpoint_indices: Vec<(usize, usize)>,
    vertex_indices: Vec<usize>,
    index_of_closest: usize,
    index_of_furthest: usize,
    domain: Range<Vec2>,
}

// goofy spacing to preserve order after auto-formatting.
// Mostly just important for docs.

mod meta;

mod indexical;

mod segmental;

mod directional;

mod positional;

mod maps_and_filters;

mod filter;
pub use filter::Filter;
