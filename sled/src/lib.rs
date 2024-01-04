//! A crate for spatial control of LED strip light configurations.
//!
//! Quick Links:
//! - [Sled] spatial read/write methods for our system
//! - [Led] Struct representing each LED in our system
//! - [color::Rgb] 32-bit/channel color representation provided by [palette](https://crates.io/crates/palette)
//! - [glam::Vec2] 2D vector struct provided by [glam](https://crates.io/crates/glam)
//!
//! # Basic Setup
//! To create a [Sled] struct, you need to create a configuration file and pass the file path in to the constructor:
//! ```rust, no_run
//! # use sled::Sled;
//! # fn main() -> Result<(), sled::SledError> {
//! let mut sled = Sled::new("/path/to/config.toml")?;
//! # Ok(())
//! # }
//! ```
//!
//! These config files are used to map the LEDs in your setup to 2D space.
//! See [Sled::new()] for an example .toml file.
//!
//! ## Drawing
//! Once you have your Sled struct, you can start drawing to it right away!
//! Here's a taste of some of the things Sled lets you do:
//!
//! Set all vertices to white:
//! ```rust
//!# use sled::{Sled, color::Rgb};
//!# let mut sled = Sled::new("./examples/config.toml").unwrap();
//! sled.set_vertices(Rgb::new(1.0, 1.0, 1.0));
//! ```
//!
//! Set all LEDs 1 unit away from the [Sled::center_point()] to red:
//! ```rust
//!# use sled::{Sled, SledError, color::Rgb};
//!# fn main() -> Result<(), SledError> {
//!# let mut sled = Sled::new("./examples/config.toml").unwrap();
//! sled.set_at_dist(1.0, Rgb::new(1.0, 0.0, 0.0))?;
//!# Ok(())}
//! ```
//!
//! Set each LED using a function of its direction from point `(2, 1)`:
//!```rust
//!# use sled::{Sled, Vec2, color::Rgb};
//!# let mut sled = Sled::new("./examples/config.toml").unwrap();
//! sled.map_by_dir_from(Vec2::new(2.0, 1.0), |dir| {
//!     let red = (dir.x + 1.0) * 0.5;
//!     let green = (dir.y + 1.0) * 0.5;
//!     Rgb::new(red, green, 0.5)
//! });
//! ```
//!
//! Dim one of the walls by 50%:
//! ```rust
//!# use sled::{Sled, SledError};
//!# fn main() -> Result<(), SledError> {
//!# let mut sled = Sled::new("./examples/config.toml")?;
//! sled.modulate_segment(2, |led| led.color * 0.5)?;
//!# Ok(())
//!# }
//!```
//! Set all LEDs within the overlap of two different circles to blue:
//! ```rust
//!# use sled::{Sled, SledError, Vec2, Filter, color::Rgb};
//!# fn main() -> Result<(), SledError> {
//!# let mut sled = Sled::new("./examples/config.toml")?;
//! let circle_1: Filter = sled.get_within_dist_from(
//!     2.0,
//!     Vec2::new(-0.5, 0.0)
//! );
//!
//! let circle_2: Filter = sled.get_within_dist_from(
//!     1.0,
//!     Vec2::new(0.5, 0.5)
//! );
//!
//! let overlap = circle_1.and(&circle_2);
//! sled.set_filter(&overlap, Rgb::new(0.0, 0.0, 1.0));
//!# Ok(())
//!# }
//! ```
//! For more examples, see the page for the [Sled] struct.
//!
//! ## Output
//! Once you're ready to display these colors, you'll probably want them packed in a nice contiguous array of RGB values. There are a few methods available to pack germane data.
//! ```rust
//!# use sled::{Sled, Vec2, color::Rgb};
//!# let sled = Sled::new("./examples/config.toml").unwrap();
//! let colors_f32: Vec<Rgb> = sled.read_colors();
//! let colors_u8: Vec<Rgb<_, u8>> = sled.read_colors();
//! 
//! let positions: Vec<Vec2> = sled.read_positions();
//!
//! let colors_and_positions: Vec<(Rgb, Vec2)> =
//!     sled.read_colors_and_positions();
//! ```
//! Or, if you just want the [Led] structs raw...
//! ```rust
//!# use sled::{Sled, Led};
//!# let sled = Sled::new("./examples/config.toml").unwrap();
//! let leds: Vec<Led> = sled.read();
//! ```

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
/// Using [glam](https://crates.io/crates/glam)'s implementation.
pub use glam::Vec2;
pub use led::Led;
pub use sled::Filter;
pub use sled::Sled;
