use std::ops::Range;

use crate::{config::LineSegment, led::Led, Vec2};

#[allow(dead_code)]
pub struct Sled {
    center_point: Vec2,
    leds: Vec<Led>,
    num_leds: usize,
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
