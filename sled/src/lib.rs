pub mod color;
pub mod error;
pub mod sled;

pub use sled::{Sled, led::Led};
pub use error::SledError;
pub use color::Rgb;

