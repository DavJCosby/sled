use core::time::Duration;

use alloc::boxed::Box;

use crate::{color::ColorType, time::Instant, Led, Sled, SledError, Vec2};

/// A driver representing instants with `std::time::Instant`
#[cfg(feature = "std")]
pub type Driver<COLOR> = CustomDriver<std::time::Instant, COLOR>;

mod data;
pub use data::Data;

#[derive(Clone, Debug)]
pub struct Time {
    pub elapsed: Duration,
    pub delta: Duration,
}

type SledResult = Result<(), SledError>;

/// Drivers are useful for encapsulating everything you need to drive a complicated lighting effect all in one place.
///
/// Some [macros](crate::driver_macros) have been provided to make authoring drivers a more ergonomic experience. See their doc comments for more information.
pub struct CustomDriver<INSTANT, COLOR>
where
    INSTANT: Instant,
    COLOR: ColorType,
{
    sled: Option<Sled<COLOR>>,
    startup_commands: Box<dyn Fn(&mut Sled<COLOR>, &mut Data) -> SledResult>,
    compute_commands: Box<dyn Fn(&Sled<COLOR>, &mut Data, &Time) -> SledResult>,
    draw_commands: Box<dyn Fn(&mut Sled<COLOR>, &Data, &Time) -> SledResult>,
    startup: INSTANT,
    last_update: INSTANT,

    data: Data,
}

impl<INSTANT, COLOR> CustomDriver<INSTANT, COLOR>
where
    INSTANT: Instant,
    COLOR: ColorType,
{
    pub fn new() -> Self {
        CustomDriver {
            sled: None,
            startup_commands: Box::new(|_, _| Ok(())),
            compute_commands: Box::new(|_, _, _| Ok(())),
            draw_commands: Box::new(|_, _, _| Ok(())),
            startup: INSTANT::now(),
            last_update: INSTANT::now(),
            data: Data::new(),
        }
    }

    /// Returns `Some(&Sled)` if the Driver has been mounted, `None` if it hasn't.
    pub fn sled(&self) -> Option<&Sled<COLOR>> {
        self.sled.as_ref()
    }

    /// Returns a duration representing how long it has been since the Driver was initially [mounted](Driver::mount).
    pub fn elapsed(&self) -> Duration {
        self.startup.elapsed()
    }

    /// Define commands to be called as soon as a Sled is [mounted](CustomDriver::mount) to the driver. This is a good place to initialize important buffer values.
    /// ```rust
    /// # use spatial_led::{Vec2, Sled, SledResult, driver::{Driver, Data}};
    /// # use palette::rgb::Rgb;
    /// use spatial_led::driver_macros::*;
    ///
    /// # // #[startup_commands]
    /// fn startup(_: &mut Sled<Rgb>, data: &mut Data) -> SledResult {
    ///     let streak_positions = vec![
    ///         Vec2::new(-1.2, 0.3),
    ///         Vec2::new(0.9, 1.6),
    ///         Vec2::new(0.4, -2.3),
    ///     ];
    ///     
    ///     data.set("positions", streak_positions);
    ///     Ok(())
    /// }
    ///
    /// pub fn main() {
    ///     let mut driver = Driver::<Rgb>::new();
    ///     driver.set_startup_commands(startup);
    /// }
    /// ```
    pub fn set_startup_commands<F: Fn(&mut Sled<COLOR>, &mut Data) -> SledResult + 'static>(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    /// Define commands to be called each time [CustomDriver::step()] is called, right before we run [draw commands](CustomDriver::set_draw_commands).
    /// ```rust
    ///# use spatial_led::{Vec2, Sled, SledResult, driver::{Driver, Data, Time}};
    ///# use palette::rgb::Rgb;
    /// use spatial_led::driver_macros::*;
    /// const WIND: Vec2 = Vec2::new(0.25, 1.5);
    ///
    /// # // #[compute_commands]
    /// fn compute(_: &Sled<Rgb>, data: &mut Data, time: &Time) -> SledResult {
    ///     let streak_positions: &mut Vec<Vec2> = data.get_mut("positions")?;
    ///     let elapsed = time.elapsed.as_secs_f32();
    ///     for p in streak_positions {
    ///         *p += WIND * elapsed
    ///     }
    ///    Ok(())
    /// }
    ///
    /// pub fn main() {
    ///     let mut driver = Driver::<Rgb>::new();
    ///     driver.set_compute_commands(compute);
    /// }
    ///
    /// ```
    pub fn set_compute_commands<
        F: Fn(&Sled<COLOR>, &mut Data, &Time) -> SledResult + 'static,
    >(
        &mut self,
        compute_commands: F,
    ) {
        self.compute_commands = Box::new(compute_commands);
    }

    /// Define commands to be called each time [CustomDriver::step()] is called, right after we run [compute commands](CustomDriver::set_compute_commands).
    /// ```rust
    /// # use spatial_led::{Sled, Vec2, SledResult, driver::{Driver, Time, Data}};
    /// # use palette::rgb::Rgb;
    /// use spatial_led::driver_macros::*;
    ///
    /// fn draw(sled: &mut Sled<Rgb>, data: &Data, _:&Time) -> SledResult {
    ///     // gradually fade all LEDs to black
    ///     sled.map(|led| led.color * 0.95);
    ///
    ///     // For each position in our buffer, draw  white in the direction to it.
    ///     let streak_positions = data.get::<Vec<Vec2>>("positions")?;
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
    pub fn set_draw_commands<F: Fn(&mut Sled<COLOR>, &Data, &Time) -> SledResult + 'static>(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    /// Takes ownership of the given Sled and runs the Driver's [startup commands](Driver::set_startup_commands).
    pub fn mount(&mut self, mut sled: Sled<COLOR>) {
        (self.startup_commands)(&mut sled, &mut self.data).unwrap();
        self.startup = INSTANT::now();
        self.last_update = self.startup;
        self.sled = Some(sled);
    }

    /// Runs the Driver's [compute commands](CustomDriver::set_compute_commands) first, and then runs its [draw commands](CustomDriver::set_draw_commands).
    pub fn step(&mut self) {
        if let Some(sled) = &mut self.sled {
            let time = Time {
                elapsed: self.startup.elapsed(),
                delta: self.last_update.elapsed(),
            };

            self.last_update = INSTANT::now();
            (self.compute_commands)(sled, &mut self.data, &time).unwrap();
            (self.draw_commands)(sled, &self.data, &time).unwrap();
        }
    }

    pub fn step_by(&mut self, delta: Duration) {
        self.startup -= delta;
        self.step();
    }

    /// Returns full ownership over the Driver's assigned Sled. Panics if [Driver::mount()] was never called.
    pub fn dismount(&mut self) -> Sled<COLOR> {
        self.sled.take().unwrap()
    }

    /// See [Sled::leds()].
    pub fn leds(&self) -> impl Iterator<Item = &Led<COLOR>> {
        if let Some(sled) = &self.sled {
            sled.leds()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// See [Sled::colors()].
    pub fn colors(&self) -> impl Iterator<Item = &COLOR> + '_ {
        if let Some(sled) = &self.sled {
            sled.colors()
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

    pub fn colors_and_positions(&self) -> impl Iterator<Item = (COLOR, Vec2)> + '_ {
        if let Some(sled) = &self.sled {
            sled.colors_and_positions()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    /// Returns a reference to the Driver's BufferContainer. Helpful for displaying buffer values to the program user.
    pub fn data(&self) -> &Data {
        &self.data
    }

    /// Returns a mutable reference to the Driver's BufferContainer. Helpful for changing buffer values as the user provides input to the program.
    pub fn data_mut(&mut self) -> &mut Data {
        &mut self.data
    }
}

impl<INSTANT, COLOR> Default for CustomDriver<INSTANT, COLOR>
where
    INSTANT: Instant,
    COLOR: ColorType,
{
    fn default() -> Self {
        Self::new()
    }
}
