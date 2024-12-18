use core::ops::Range;

use alloc::vec::Vec;

use crate::{color::ColorType, config::LineSegment, led::Led, Vec2};

#[allow(dead_code)]
#[derive(Clone, Debug)]
/// A struct representing the layout of some LED configuration in 2D space, composed of line segments.
///
/// Sled structs are [constructed](Sled::new) from a .toml file that describe this layout.
/// Upon construction, key information like the indices of vertices or the angle from each led from the center_point is precalculated and cached for faster access later.
/// Sled takes a generic type parameter `COLOR` to define how you want to store color information. Whatever you set this to, all draw methods will expect to see. Documentation examples uses palette's [palette's Rgb struct](https://docs.rs/palette/latest/palette/rgb/struct.Rgb.html) struct, but you can use any data type that implements `Debug`, `Default`, and `Copy`.
/// ```rust, ignore
/// #[derive(Debug)]
/// struct RGBW {
///     r: f32,
///     g: f32,
///     b: f32,
///     w: f32
/// }
/// let mut u8_sled = Sled::<(u8, u8, u8)>::new("/path/to/config.yap")?;
/// let mut rgbw_sled = Sled::<RGBW>::new("/path/to/config.yap")?;
///
/// u8_sled.set(4, (255, 0, 0))?; // set 5th led to red
/// rgbw_sled.set_all(RGBW {
///     r: 0.0,
///     g: 1.0,
///     b: 0.0,
///     w: 0.0
/// });
/// ```
pub struct Sled<COLOR: ColorType> {
    center_point: Vec2,
    leds: Vec<Led<COLOR>>,
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
