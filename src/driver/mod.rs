use core::time::Duration;

use alloc::boxed::Box;

use crate::{
    color::{Rgb, Srgb},
    time::Instant,
    Led, Sled, SledError, Vec2,
};

/// A driver representing instants with `std::time::Instant`
#[cfg(feature = "std")]
pub type Driver = CustomDriver<std::time::Instant>;

mod filters;
// mod sliders;
mod buffers;
pub use buffers::BufferContainer;
pub use filters::Filters;

#[derive(Clone, Debug)]
pub struct TimeInfo {
    pub elapsed: Duration,
    pub delta: Duration,
}

type SledResult = Result<(), SledError>;
type StartupCommands = Box<dyn Fn(&mut Sled, &mut BufferContainer, &mut Filters) -> SledResult>;
type ComputeCommands =
    Box<dyn Fn(&Sled, &mut BufferContainer, &mut Filters, &TimeInfo) -> SledResult>;
type DrawCommands = Box<dyn Fn(&mut Sled, &BufferContainer, &Filters, &TimeInfo) -> SledResult>;

/// Drivers are useful for encapsulating everything you need to drive a complicated lighting effect all in one place.
///
/// Some [macros](crate::driver_macros) have been provided to make authoring drivers a more ergonomic experience. See their doc comments for more information.
pub struct CustomDriver<INSTANT>
where
    INSTANT: Instant,
{
    sled: Option<Sled>,
    startup_commands: StartupCommands,
    compute_commands: ComputeCommands,
    draw_commands: DrawCommands,
    startup: INSTANT,
    last_update: INSTANT,
    buffers: BufferContainer,
    filters: Filters,
}

impl<INSTANT> CustomDriver<INSTANT>
where
    INSTANT: Instant,
{
    pub fn new() -> Self {
        CustomDriver {
            sled: None,
            startup_commands: Box::new(|_, _, _| Ok(())),
            compute_commands: Box::new(|_, _, _, _| Ok(())),
            draw_commands: Box::new(|_, _, _, _| Ok(())),
            startup: INSTANT::now(),
            last_update: INSTANT::now(),
            buffers: BufferContainer::new(),
            filters: Filters::new(),
        }
    }

    /// Returns `Some(&Sled)` if the Driver has been mounted, `None` if it hasn't.
    pub fn sled(&self) -> Option<&Sled> {
        self.sled.as_ref()
    }

    /// Returns a duration representing how long it has been since the Driver was initially [mounted](Driver::mount).
    pub fn elapsed(&self) -> Duration {
        self.startup.elapsed()
    }

    /// Define commands to be called as soon as a Sled is [mounted](CustomDriver::mount) to the driver. This is a good place to initialize important buffer values.
    /// ```rust
    /// # use spatial_led::{Vec2, BufferContainer, SledResult, driver::Driver};
    /// use spatial_led::driver_macros::*;
    ///
    /// #[startup_commands]
    /// fn startup(buffers: &mut BufferContainer) -> SledResult {
    ///     let streak_positions = buffers.create_buffer::<Vec2>("positions");
    ///     streak_positions.extend([
    ///         Vec2::new(-1.2, 0.3),
    ///         Vec2::new(0.9, 1.6),
    ///         Vec2::new(0.4, -2.3),
    ///     ]);
    ///     Ok(())
    /// }
    ///
    /// pub fn main() {
    ///     let mut driver = Driver::new();
    ///     driver.set_startup_commands(startup);
    /// }
    /// ```
    pub fn set_startup_commands<
        F: Fn(&mut Sled, &mut BufferContainer, &mut Filters) -> SledResult + 'static,
    >(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    /// Define commands to be called each time [CustomDriver::step()] is called, right before we run [draw commands](CustomDriver::set_draw_commands).
    /// ```rust
    /// # use spatial_led::{Vec2, BufferContainer, TimeInfo, SledResult, driver::Driver};
    /// use spatial_led::driver_macros::*;
    /// const WIND: Vec2 = Vec2::new(0.25, 1.5);
    ///
    /// #[compute_commands]
    /// fn compute(buffers: &mut BufferContainer, time: &TimeInfo) -> SledResult {
    ///     let streak_positions = buffers.get_buffer_mut::<Vec2>("positions")?;
    ///     let elapsed = time.elapsed.as_secs_f32();
    ///     for p in streak_positions {
    ///         *p += WIND * elapsed
    ///     }
    ///    Ok(())
    /// }
    ///
    /// pub fn main() {
    ///     let mut driver = Driver::new();
    ///     driver.set_compute_commands(compute);
    /// }
    ///
    /// ```
    pub fn set_compute_commands<
        F: Fn(&Sled, &mut BufferContainer, &mut Filters, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        compute_commands: F,
    ) {
        self.compute_commands = Box::new(compute_commands);
    }

    /// Define commands to be called each time [CustomDriver::step()] is called, right after we run [compute commands](CustomDriver::set_compute_commands).
    /// ```rust
    /// # use spatial_led::{Sled, Vec2, color::Rgb, BufferContainer, TimeInfo, SledResult, driver::Driver};
    /// use spatial_led::driver_macros::*;
    ///
    /// #[draw_commands]
    /// fn draw(sled: &mut Sled, buffers: &BufferContainer) -> SledResult {
    ///     // gradually fade all LEDs to black
    ///     sled.map(|led| led.color * 0.95);
    ///
    ///     // For each position in our buffer, draw  white in the direction to it.
    ///     let streak_positions = buffers.get_buffer::<Vec2>("positions")?;
    ///     let center = sled.center_point();
    ///     for pos in streak_positions {
    ///         let dir = (pos - center).normalize();
    ///         sled.set_at_dir(dir, Rgb::new(1.0, 1.0, 1.0));
    ///     }
    ///    Ok(())
    /// }
    ///
    /// pub fn main() {
    ///     let mut driver = Driver::new();
    ///     driver.set_draw_commands(draw);
    /// }
    ///
    /// ```
    pub fn set_draw_commands<
        F: Fn(&mut Sled, &BufferContainer, &Filters, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    /// Takes ownership of the given Sled and runs the Driver's [startup commands](CustomDriver::set_startup_commands).
    pub fn mount(&mut self, mut sled: Sled) {
        (self.startup_commands)(&mut sled, &mut self.buffers, &mut self.filters).unwrap();
        self.startup = INSTANT::now();
        self.last_update = self.startup;
        self.sled = Some(sled);
    }

    /// Runs the Driver's [compute commands](CustomDriver::set_compute_commands) first, and then runs its [draw commands](CustomDriver::set_draw_commands).
    pub fn step(&mut self) {
        if let Some(sled) = &mut self.sled {
            let time_info = TimeInfo {
                elapsed: self.startup.elapsed(),
                delta: self.last_update.elapsed(),
            };

            self.last_update = INSTANT::now();
            (self.compute_commands)(sled, &mut self.buffers, &mut self.filters, &time_info)
                .unwrap();
            (self.draw_commands)(sled, &self.buffers, &self.filters, &time_info).unwrap();
        }
    }

    pub fn step_by(&mut self, delta: Duration) {
        self.startup -= delta;
        self.step();
    }

    /// Returns full ownership over the Driver's assigned Sled. Panics if [CustomDriver::mount()] was never called.
    pub fn dismount(&mut self) -> Sled {
        self.sled.take().unwrap()
    }

    /// See [Sled::leds()].
    pub fn leds(&self) -> impl Iterator<Item = &Led> {
        if let Some(sled) = &self.sled {
            sled.leds()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// See [Sled::colors()].
    pub fn colors(&self) -> impl Iterator<Item = &Rgb> + '_ {
        if let Some(sled) = &self.sled {
            sled.colors()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// See [Sled::colors_coerced()].
    pub fn colors_coerced<T>(&self) -> impl Iterator<Item = Srgb<T>> + '_
    where
        f32: crate::color::stimulus::IntoStimulus<T>,
    {
        if let Some(sled) = &self.sled {
            sled.colors_coerced()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// See [Sled::positions()].
    pub fn positions(&self) -> impl Iterator<Item = Vec2> + '_ {
        if let Some(sled) = &self.sled {
            sled.positions()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// See [Sled::colors_and_positions_coerced()].
    pub fn colors_and_positions_coerced<T>(&self) -> impl Iterator<Item = (Srgb<T>, Vec2)> + '_
    where
        f32: crate::color::stimulus::IntoStimulus<T>,
    {
        if let Some(sled) = &self.sled {
            sled.colors_and_positions_coerced()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// Returns a reference to the Driver's BufferContainer. Helpful for displaying buffer values to the program user.
    pub fn buffers(&self) -> &BufferContainer {
        &self.buffers
    }

    /// Returns a mutable reference to the Driver's BufferContainer. Helpful for changing buffer values as the user provides input to the program.
    pub fn buffers_mut(&mut self) -> &mut BufferContainer {
        &mut self.buffers
    }
}

impl<INSTANT> Default for CustomDriver<INSTANT>
where
    INSTANT: Instant,
{
    fn default() -> Self {
        Self::new()
    }
}
