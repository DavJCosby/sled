//! A crate for spatial control of LED strip light configurations.
//!
//! Quick Links:
//! - [Sled] spatial read/write methods for our system
//! - [Led] Struct representing each LED in our system
//! - [color::Rgb] 32-bit/channel color representation provided by [palette](https://crates.io/crates/palette)
//! - [glam::Vec2] 2D vector struct provided by [glam](https://crates.io/crates/glam)
//!
//! # Basic Setup
//! To create a Sled struct, you need to create a configuration file and pass the file path in to the constructor:
//! ```rust, ignore
//! let mut sled = Sled::new("/path/to/config.toml").unwrap();
//! ````
//!
//! These config files are used to map the LEDs in your setup to 2D space. Here's an example .toml file:
//!
//! ```ignore
//! // config.toml
//! center_point = [0.0, 0.5]
//! density = 30.0
//!
//! [[line_segment]]
//! start = [-2.0, 0.0]
//! end = [0.5, -1.0]
//!
//! [[line_segment]]
//! start = [0.5, -1.0]
//! end = [3.5, 0.0]
//!
//! [[line_segment]]
//! start = [3.5, 0.0]
//! end = [2, 2]
//!
//! [[line_segment]]
//! start = [2.0, 2]
//! end = [-2.0, 2]
//!
//! [[line_segment]]
//! start = [-2.0, 2]
//! end = [-2.0, 0.0]
//! ```
//! `center_point` is a static reference point you can use to speed up draw calls.
//! At initialization, directions, distances, etc relative to this point are pre-calculated for each Led.
//!
//! `density` represents how many LED's per unit we can expect for the line segments below. If one or more
//! LED has a different density for whatever reason, you can override this default for each line_segment.
//!
//! Add as many `[[line_segment]]`s as you need to represent your scene.
//!
//! ## Drawing
//! Once you have your Sled struct, you can start drawing to it right away!
//! Here's a taste of some of the things Sled lets you do:
//!
//! Set all vertices to white
//! ```ignore
//! sled.set_vertices(Rgb::new(1.0, 1.0, 1.0));
//! ```
//! > Set all LEDs within 1 unit of the center_point to red
//! ```ignore
//! sled.set_within_dist(1.0, Rgb::new(1.0, 0.0, 0.0));
//! ```
//! Set each LED using a function of its direction from the point `(2, 1)`
//! ```rust, ignore
//! sled.map_by_dir_from(Vec2::new(2.0, 1.0), |dir| {
//!     let red = (dir.x + 1.0) * 0.5;
//!     let green = (dir.y + 1.0) * 0.5;
//!     Rgb::new(red, green, 0.5)
//! });
//! ```
//! Dim one of our walls by 50%
//! ```rust, ignore
//! sled.modulate_segment(2, |led| led.color * 0.5)?;
//! ```
//! For more examples see the page for the [Sled] struct.
//!
//! ## Output
//! Once you're ready to display these colors, you'll probably want them packed in a nice contiguous array of RGB values. With Sled, that's as easy as:
//! ```rust, ignore
//!     let colors: Vec<Rgb<_, u8>> = sled.read_colors();
//! ```
//! > *Note, the example about automatically converts the Rgbs to a 0-255 scale (8 bits/channel).
//! > Replace the u8 with f32 if you need a 0.0 - 1.0 scale (32 bits/channel).*

pub mod color;
mod config;
mod error;
mod led;
mod sled;

#[cfg(feature = "drivers")]
pub mod driver;

#[cfg(feature = "scheduler")]
pub mod scheduler;

pub use error::SledError;
/// [glam](https://crates.io/crates/glam)'s implementation.
pub use glam::Vec2;
pub use led::Led;
pub use sled::Filter;
pub use sled::Sled;
