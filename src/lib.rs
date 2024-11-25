#![cfg_attr(not(feature = "std"), no_std)]

//! # Spatial LED (Sled)
//! <div> <img src="https://github.com/DavJCosby/sled/blob/master/resources/ripples-demo.gif?raw=true" width="49%" title="cargo run --example ripples"> <img src="https://github.com/DavJCosby/sled/blob/master/resources/warpspeed-demo.gif?raw=true" width="49%" title="cargo run --example warpspeed">
//! </div>
//!
//! Sled is a rust library for creating spatial lighting effects for individually addressable LED strips. API ergonomics and performance are top priorities for this project. That said, Sled is still in its early stages of development which means there is plenty of room for improvement in both categories.
//!
//! What Sled **does** do:
//! - It exposes an API that lets you:
//!     - Modify virtual LED colors depending on each LED's position, distance, direction, line segment, etc;
//!     - Output that color data in a simple, contiguous data structure for your own usage;
//!     - Filter LEDs by their spatial properties to pre-compute important sets;
//! - Additionally, some tools are provided to help you build functional apps faster (you may opt-out with compiler features):
//!     - `Driver` - Pack draw/compute logic into a Driver to simplify to the process of swapping between effects, or changing effect settings at runtime.
//!     - `Scheduler` - Lightweight tool to schedule redraws at a fixed rate, powered by [spin_sleep](https://github.com/alexheretic/spin-sleep).
//!
//! What Sled **does not** do:
//! - It does not interface directly with your GPIO pins to control your LED hardware. Each project will be different, so it's up to you to bring your own glue. Check out the [Raspberry Pi example](https://github.com/DavJCosby/spatial_led_examples/tree/main/raspberry_pi) to get an idea what that might look like.
//! - It does not allow you to represent your LEDs in 3D space. Could be a fun idea in the future, but it's just not planned for the time being.
//!
//! See the [spatial_led_examples](https://github.com/DavJCosby/spatial_led_examples) repository for examples of Sled in action!
//!
//! ## The Basics
//! In absence of an official guide, this will serve as a basic introduction to Sled. From here, you can use the documentation comments to learn what else Sled offers.
//! ### Setup
//! To [create](Sled::new) a [Sled] struct, you need to create a configuration file and provide its path to the constructor:
//! ```rust, ignore
//! use spatial_led::Sled;
//! let mut sled = Sled::new("/path/to/config.yap")?;
//! ```
//!
//! A configuration file explains the layout of your LED strips in 2D space. This is used to pre-calculate some important information that's used to speed up complex draw calls.
//!
//! Example file:
//! ```yaml, no_run
//! center: (0.0, 0.5)
//! density: 30.0
//! --segments--
//! (-2, 0) --> (0.5, -1) --> (3.5, 0) -->
//! (2, 2) --> (-2, 2) --> (-2, 0)
//! ```
//! See [Sled::new()] for more information on this config format.
//!
//! ### Drawing
//! Once you have your [Sled] struct, you can start drawing to it right away! Here’s a taste of some of the things Sled lets you do:
//!
//! **Set all vertices to white:**
//! ```rust
//! # use spatial_led::{Sled, color::Rgb};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! sled.set_vertices(Rgb::new(1.0, 1.0, 1.0));
//! ```
//! ![Set all Vertices](https://github.com/DavJCosby/sled/blob/master/resources/vertices.png?raw=true)
//! > Note that this is a custom terminal UI visualization that is not packaged as part of the sled crate. It is ultimately up to you to decide how to visualize your LEDs, Sled just handles the computation.
//!
//! **Set all LEDs 2 units away from the `center_point` to red:**
//! ```rust
//! # use spatial_led::{Sled, color::Rgb};
//! # fn main() -> Result<(), spatial_led::SledError> {
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! sled.set_at_dist(2.0, Rgb::new(1.0, 0.0, 0.0));
//! # Ok(())
//! # }
//! ```
//! ![Set at Distance](https://raw.githubusercontent.com/DavJCosby/sled/master/resources/at_distance.png)
//!
//! **Set each LED using a function of its direction from point `(2, 1)`:**
//! ```rust
//! # use spatial_led::{Sled, Vec2, color::Rgb};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//!  sled.map_by_dir_from(Vec2::new(2.0, 1.0), |dir| {
//!      let red = (dir.x + 1.0) * 0.5;
//!      let green = (dir.y + 1.0) * 0.5;
//!      Rgb::new(red, green, 0.5)
//!  });
//! ```
//! ![Map by Direction](https://github.com/DavJCosby/sled/blob/master/resources/dir_map.png?raw=true)
//!
//! **Dim one of the walls by 75%:**
//! ```rust
//! # use spatial_led::{Sled};
//! # fn main() -> Result<(), spatial_led::SledError> {
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! sled.modulate_segment(3, |led| led.color * 0.25)?;
//! # Ok(())
//! # }
//! ```
//! ![Modulate Segment](https://github.com/DavJCosby/sled/blob/master/resources/segment_modulate.png?raw=true)
//!
//! **Set all LEDs within the overlapping areas of two different circles to blue:**
//! ```rust
//! # use spatial_led::{Sled, Filter, Vec2, color::Rgb};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! let circle_1: Filter = sled.within_dist_from(
//!     2.0,
//!     Vec2::new(1.0, 0.5)
//! );
//!     
//! let circle_2: Filter = sled.within_dist_from(
//!     2.5,
//!     Vec2::new(-1.0, 1.5)
//! );
//!
//! let overlap = circle_1.and(&circle_2);
//! sled.set_filter(&overlap, Rgb::new(0.0, 0.0, 1.0));
//! ```
//! ![Set Overlapping Areas](https://github.com/DavJCosby/sled/blob/master/resources/filter_and.png?raw=true)
//! For more examples, see the documentation comments on the [Sled] struct.
//!
//! ## Output
//!
//! Once you’re ready to display these colors, you’ll probably want them packed in a nice contiguous array of [RGB](color::Rgb) values. There are a few methods available to pack the information you need.
//!
//! ```rust
//! # use spatial_led::{Sled, Vec2, color::Rgb};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! // An Iterator of Rgbs, 32-bits/channel
//! let colors_f32 = sled.colors();
//!
//! for color in colors_f32 {
//!     let red: f32 = color.red;
//!     // -snip- //
//! }
//! ```
//!
//! A few other handy output methods:
//!
//! ```rust
//! # use spatial_led::{Sled, Vec2, color::Rgb};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! // An Iterator of Rgbs, 8-bits/channel (overhead for conversion)
//! let colors_u8 = sled.colors_coerced::<u8>();
//! // An Iterator of Vec2s, representing the position of each leds
//! let positions = sled.positions();
//! // An Iterator of (Rgb, Vec2) tuple pairs representing each leds color and position.
//! let colors_f32_and_positions = sled.colors_and_positions();
//! // An Iterator of (Rgb<u8>, Vec2) tuple pairs representing each leds color and position.
//! let colors_f32_and_positions = sled.colors_and_positions_coerced::<u8>();
//! ```
//!
//! # Advanced Features
//! For basic applications, the [Sled] struct gives you plenty of power. Odds are though, you'll want to create more advanced effects that might be time or user-input driven. A few optional (enabled by default, opt-out by disabling their compiler features) tools are provided to streamline that process.
//!
//! ## Drivers
//! [Drivers](driver::Driver) are useful for encapsulating everything you need to drive a lighting effect all in one place. Here's an example of what a simple, time-based one might look like:
//!
//! ```rust
//! # use spatial_led::{Sled, color::Rgb};
//! use spatial_led::driver::Driver;
//! let mut driver = Driver::new();
//!
//! driver.set_startup_commands(|_sled, buffers, _filters| {
//!     let colors = buffers.create_buffer::<Rgb>("colors");
//!     colors.extend([
//!         Rgb::new(1.0, 0.0, 0.0),
//!         Rgb::new(0.0, 1.0, 0.0),
//!         Rgb::new(0.0, 0.0, 1.0),
//!     ]);
//!     Ok(())
//! });
//!
//! driver.set_draw_commands(|sled, buffers, _filters, time_info| {
//!     let elapsed = time_info.elapsed.as_secs_f32();
//!     let colors: &Vec<Rgb> = buffers.get_buffer("colors")?;
//!     let num_colors = colors.len();
//!     // clear our canvas each frame
//!     sled.set_all(Rgb::new(0.0, 0.0, 0.0));
//!
//!     for i in 0..num_colors {
//!         let alpha = i as f32 / num_colors as f32;
//!         let angle = elapsed + (std::f32::consts::TAU * alpha);
//!         sled.set_at_angle(angle, colors[i]);
//!     }
//!
//!     Ok(())
//! });
//! ```
//! To start using the Driver, give it ownership over a Sled using [.mount()](driver::Driver::mount) and use [.step()](driver::Driver::step) to manually refresh it.
//! ```rust, no_run
//! # use spatial_led::{Sled, driver::Driver};
//! # fn main() -> Result<(), spatial_led::SledError> {
//! let sled = Sled::new("path/to/config.yap")?;
//! # let mut driver = Driver::new();
//! driver.mount(sled); // sled gets moved into driver here.
//!
//! loop {
//!     driver.step();
//!     let colors = driver.colors();
//!     // display those colors however you want
//! }
//!
//! # Ok(())
//! # }
//! ```
//! ![Basic Time-Driven Effect Using Buffers](https://github.com/DavJCosby/sled/blob/master/resources/driver1.gif?raw=true)
//!
//! If you need to retrieve ownership of your sled later, you can do:
//! ```rust
//! # use spatial_led::{Sled, driver::Driver};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! # let mut driver = Driver::new();
//! # driver.mount(sled);
//! let sled = driver.dismount();
//! ```
//!
//! * [set_startup_commands()](driver::Driver::set_startup_commands) - Define a function or closure to run when `driver.mount()` is called. Grants mutable control over [Sled], [BufferContainer], and [Filters].
//!
//! * [set_draw_commands()](driver::Driver::set_draw_commands) - Define a function or closure to run every time `driver.step()` is called. Grants mutable control over `Sled`, and immutable access to `BufferContainer`, `Filters`, and `TimeInfo`.
//!
//! * [set_compute_commands()](driver::Driver::set_compute_commands) - Define a function or closure to run every time `driver.step()` is called, scheduled right before draw commands. Grants immutable access to `Sled`, mutable control over `BufferContainer` and `Filters` and immutable access to `TimeInfo`.
//!
//! Drivers need a representation of a time instant, which is provided as a generic `INSTANT` that must implement the trait `time::Instant`. For `std` targets, `std::time::Instant` can be used, and a type alias `Driver = CustomDriver<std::time::Instant>` is defined. For `no_std` targets, the client should define their own representation (e.g. using `embassy_time::Instant`).
//!
//! If you don't want to Drivers for your project, you can disable the `drivers` compiler feature to shed a couple dependencies.
//!
//! For more examples of ways to use drivers, see the [driver_examples folder](https://github.com/DavJCosby/spatial_led_examples/tree/main/driver_examples) in the spatial_led_examples repository.
//!
//! ### Driver Macros
//! Some [macros](driver_macros) have been provided to make authoring drivers a more ergonomic experience. You can apply the following attributes to functions that you want to use for driver commands:
//! * `#[startup_commands]`
//! * `#[compute_commands]`
//! * `#[draw_commands]`
//!
//! Using these, you can express your commands as a function that only explicitly states the parameters it needs. The previous example could be rewritten like this, for example:
//! ```rust
//! # use spatial_led::{Sled, driver::Driver, color::Rgb};
//! # use spatial_led::{BufferContainer, SledResult, TimeInfo};
//! use spatial_led::driver_macros::*;
//!
//! #[startup_commands]
//! fn startup(buffers: &mut BufferContainer) -> SledResult {
//!     let colors = buffers.create_buffer::<Rgb>("colors");
//!     colors.extend([
//!        Rgb::new(1.0, 0.0, 0.0),
//!        Rgb::new(0.0, 1.0, 0.0),
//!        Rgb::new(0.0, 0.0, 1.0),
//!    ]);
//!    Ok(())
//! }
//!
//! #[draw_commands]
//! fn draw(sled: &mut Sled, buffers: &BufferContainer, time_info: &TimeInfo) -> SledResult {
//!    let elapsed = time_info.elapsed.as_secs_f32();
//!    let colors = buffers.get_buffer::<Rgb>("colors")?;
//!    let num_colors = colors.len();
//!    // clear our canvas each frame
//!    sled.set_all(Rgb::new(0.0, 0.0, 0.0));
//!
//!    for i in 0..num_colors {
//!        let alpha = i as f32 / num_colors as f32;
//!        let angle = elapsed + (std::f32::consts::TAU * alpha);
//!        sled.set_at_angle(angle, colors[i]);
//!    }
//!    Ok(())
//! }
//!
//! //--snip--/
//!
//! let mut driver = Driver::new();
//! driver.set_startup_commands(startup);
//! driver.set_draw_commands(draw);
//! ```
//!
//! ### Buffers
//! A driver exposes a data structure called [BufferContainer]. A BufferContainer essentially acts as a HashMap of `&str` keys to Vectors of any type you choose to instantiate. This is particularly useful for passing important data and settings in to the effect.
//!
//! It's best practice to create buffers with [startup commands](driver::Driver::set_startup_commands), and then modify them either through [compute commands](driver::Driver::set_compute_commands) or from [outside the driver](driver::Driver::buffers_mut) depending on your needs.
//!
//! ```rust
//! # use spatial_led::{Sled, driver::{BufferContainer, Filters, Driver}, SledResult, color::Rgb};
//! # use spatial_led::driver_macros::*;
//! # type MY_CUSTOM_TYPE = f32;
//! #[startup_commands]
//! fn startup(sled: &mut Sled, buffers: &mut BufferContainer) -> SledResult {
//!     let wall_toggles: &mut Vec<bool> = buffers.create_buffer("wall_toggles");
//!     let wall_colors: &mut Vec<Rgb> = buffers.create_buffer("wall_colors");
//!     let some_important_data = buffers.create_buffer::<MY_CUSTOM_TYPE>("important_data");
//!     Ok(())
//! }
//!
//! # let mut driver = Driver::new();
//!
//! driver.set_startup_commands(startup);
//! ```
//!
//! To maniplate buffers from outside driver, just do:
//! ```rust
//! # use spatial_led::{driver::{BufferContainer, Driver}};
//! # let mut driver = Driver::new();
//! let buffers: &BufferContainer = driver.buffers();
//! // or
//! let buffers: &mut BufferContainer = driver.buffers_mut();
//! ```
//!
//! Using a BufferContainer is relatively straightforward.
//! ```rust
//! # type MY_CUSTOM_TYPE = f32;
//! # use spatial_led::{color::Rgb, Sled, driver::Driver, driver::BufferContainer};
//! # let mut driver = Driver::new();
//! driver.set_draw_commands(|sled: &mut Sled, buffers: &BufferContainer, _, _| {
//!     let wall_toggles = buffers.get_buffer::<bool>("wall_toggles")?;
//!     let wall_colors = buffers.get_buffer::<Rgb>("wall_colors")?;
//!     let important_data = buffers.get_buffer::<MY_CUSTOM_TYPE>("important_data")?;
//!
//!     for i in 0..wall_toggles.len() {
//!         if wall_toggles[i] == true {
//!             sled.set_segment(i, wall_colors[i])?;
//!        } else {
//!            sled.set_segment(i, Rgb::new(0.0, 0.0, 0.0))?;
//!        }
//!    }
//!    
//!    Ok(())
//! });
//! ```
//!
//! If you need to mutate buffer values:
//! ```rust
//!  // Mutable reference to the whole buffer
//! # use spatial_led::{driver::BufferContainer, color::Rgb, SledError};
//! # fn main() -> Result<(), spatial_led::SledError> {
//! # let mut buffers = BufferContainer::new();
//! # let mut b = buffers.create_buffer::<bool>("wall_toggles");
//! # b.push(false);
//! # b.push(false);
//! # let mut b2 = buffers.create_buffer::<Rgb>("wall_colors");
//! # b2.push(Rgb::new(1.0, 1.0, 1.0));
//! # b2.push(Rgb::new(1.0, 1.0, 1.0));
//! # b2.push(Rgb::new(1.0, 1.0, 1.0));
//! let buffer_mut = buffers.get_buffer_mut::<bool>("wall_toggles")?;
//!
//! // Modify just one item
//! buffers.set_buffer_item("wall_toggles", 1, false)?;
//!  
//! // Mutable reference to just one item
//! let color: &mut Rgb = buffers.get_buffer_item_mut("wall_colors", 2)?;
//! *color /= 2.0;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ### Filters
//! For exceptionally performance-sensitive scenarios, [Filters] can be used to predefine important LED regions. Imagine for example that we have an incredibly expensive mapping function that will only have a visible impact on the LEDs within some radius $R$ from a given point $P$.
//!
//! Rather than checking the distance of each LED from that point every frame, we can instead do something like this:
//!
//! ```rust
//! # use spatial_led::{Sled, Led, Filter, Vec2, color::Rgb, driver::Driver};
//! # let mut driver = Driver::new();
//! driver.set_startup_commands(|sled, buffers, filters| {
//!     let area: Filter = sled.within_dist_from(5.0, Vec2::new(-0.25, 1.5));
//!
//!     filters.set("area_of_effect", area);
//!     Ok(())
//! });
//!
//! driver.set_draw_commands(|sled, buffers, filters, _| {
//!     let area_filter = filters.get("area_of_effect")?;
//!     sled.map_filter(area_filter, |led| {
//!         // expensive computation
//!         # Rgb::new(0.0, 0.0, 0.0)
//!     });
//!     Ok(())
//! });
//! ```
//! Most getter methods on Sled will return a [Filter], but if you need more precise control you can do something like this:
//! ```rust
//! # use spatial_led::{Sled};
//! # let mut sled = Sled::new("./benches/config.yap").unwrap();
//! let even_filter = sled.filter(|led| led.index() % 2 == 0);
//! ```
//!
//! I imagine this feature will get less love than buffers, but I can still see a handful of scenarios where this can be very useful for some users. In a future version this may become an opt-in compiler feature.
//!
//! ## Scheduler
//! The [Scheduler](scheduler::Scheduler) struct makes it super easy to schedule redraws at a fixed rate.
//!
//! ```rust, no_run
//! # use spatial_led::{scheduler::Scheduler, driver::Driver};
//! # let mut driver = Driver::new();
//! let mut scheduler = Scheduler::new(120.0);
//!
//! scheduler.loop_forever(|| {
//!     driver.step();
//! });
//! ```
//! Scheduler, by default, utilizes [spin_sleep](https://crates.io/crates/spin_sleep/) to minimize the high CPU usage you typically see when you spin to wait for the next update.
//!
//! Here are a few other methods that you might also consider:
//!
//! ```rust, no_run
//! # use spatial_led::{scheduler::Scheduler, driver::Driver};
//! # let mut driver = Driver::new();
//! # let mut scheduler = Scheduler::new(120.0);
//! // loops until false is returned
//! scheduler.loop_while_true(|| {
//!     // -snip-
//!     return true;
//! });
//!
//! // loops until an error of any type is returned
//! scheduler.loop_until_err(|| {
//!     // -snip-
//!     Ok(())
//! });
//!
//! // best for where you don't wanna pass everything through a closure
//! loop {
//!     // -snip-
//!     scheduler.sleep_until_next_frame();
//! }
//! ```
//!
//! Schedulers need a representation of a time instant, like drivers, and also a representation of a sleep function, which is provided as a generic `SLEEPER` that must implement the trait `time::Sleeper`. For `std` targets, `std::thread::sleep()` can be used, and a type alias `Scheduler = CustomScheduler<std::time::Instant, StdSleeper>` is defined. For `no_std` targets, the client should define their own representation.
//!
//! For async environments, AsyncScheduler can be used instead. No predefined implementation is provided, the client should define their own, e.g. using `embassy_time::Timer::after().await`.
//!
//! You can define your own `CustomScheduler` backed by whatever sleeping method you prefer if you wish. If you'd like to trim away the `spin_sleep` dependency, you can also disable the `spin_sleep` feature flag.

//! If you don't need the Scheduler struct in general, you can disable the `scheduler` and `spin_sleep` flags.
//!
//! # `no_std` Support
//! Spatial LED is now usable in `no_std` environments as of 0.2.0 (though `alloc` is still required), thanks to some [awesome contributions](https://github.com/DavJCosby/sled/pull/86) by [Claudio Mattera](https://github.com/claudiomattera).
//!
//! To do this, disable the `std` flag and enable the `libm` flag (for use by glam and palette).
//!
//! Users on the nightly toolchain can also enable the `core-simd` for some extra performance if you know your target platform supports SIMD instructions.
//!
//! ## Drivers
//! The default Driver implementation depends on [std::time::Instant] to track elapsed time between driver steps. For `no_std` environments, you must provide your own struct that implements the [crate::time::Instant] trait.
//!
//! Once you have that, building a [driver::CustomDriver] becomes as easy as:
//!
//! ```rust, ignore
//! use spatial_led::driver::CustomDriver;
//!
//! let driver = CustomDriver<MyCustomInstant>::new();
//! ```
//!
//!
//! ## Schedulers
//! Similarly, the default Scheduler relies on Instants, as well as methods only available through the standard library to handle sleeping. Thus, to build a Scheduler in `no_std` environments, you'll need to provide custom implementations of the [crate::time::Instant] and [crate::time::Sleeper] traits.
//!
//! ```rust, ignore
//! use spatial_led::driver::CustomDriver;
//!
//! let scheduler = CustomScheduler<MyCustomInstant, MyCustomSleeper>::new(120.0);
//!
//! scheduler.loop_forever(|| {
//!     println!("tick!");
//! });
//! ```
//!
//! As embassy is gaining popularity in the embedded Rust scene, Claudio has also provided an async interface via the [scheduler::AsyncCustomScheduler] struct.
//!
//! ## Contributions
//! The author of this crate does not own any hardware that would allow him test spatial_led on real `no_std` environments, so bug reports and PRs are very appreciated.
//!
//! # Feature Flags
//! Enabled by Default:
//! - `std`
//! - `drivers` : Enables Drivers
//! - `scheduler` : Enables Schedulers
//! - `spin_sleep` : If `std` is enabled, sets the default Scheduler to use [spin_sleep](https://crates.io/crates/spin_sleep) to schedule tasks.
//! 
//! Opt-in:
//! - `named_colors` : Exposes color constants
//!     - (for example `spatial_led::color::consts::WHITE`)
//! - `libm` : Needed for some `no_std` environments.
//! - `core-simd` (Nightly) : Enables portable SIMD support for use by glam.
//!

extern crate alloc;
/// Exposes [palette](https://crates.io/crates/palette)'s color management tools and brings the Rgb struct forward for easier use in Sled projects.
pub mod color;
mod config;
mod error;
mod led;
mod spatial_led;

#[cfg(feature = "drivers")]
/// Useful tools for building more complicated, time-based visual effects.
///
/// Drivers are an optional feature that can be disabled by turning off the `drivers` feature flag.
pub mod driver;
#[cfg(feature = "drivers")]
pub use driver::{BufferContainer, Filters, TimeInfo};

#[cfg(feature = "drivers")] // syn may or may not use std features, need to confirm this.
pub use sled_driver_macros as driver_macros;

#[cfg(feature = "scheduler")]
/// Useful tool for scheduling redraws at a fixed rate.
///
/// Scheduler is an optional feature that can be disabled by turning off the `scheduler` feature flag.
pub mod scheduler;

pub use error::SledError;
/// Equivalent to `Result<(), SledError>`
pub type SledResult = Result<(), SledError>;
/// Using [glam](https://crates.io/crates/glam)'s implementation.
pub use glam::Vec2;
pub use led::Led;
pub use spatial_led::Filter;
pub use spatial_led::Sled;

pub mod time;
