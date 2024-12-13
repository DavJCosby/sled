use core::time::Duration;

use alloc::boxed::Box;

use crate::time::AsyncSleeper;
use crate::time::Instant;
use crate::time::Sleeper;

#[cfg(all(feature = "std", not(feature = "spin_sleep")))]
use crate::time::StdSleeper;

#[cfg(all(feature = "std", feature = "spin_sleep"))]
use crate::time::SpinSleeper;

/// A scheduler representing instants with [std::time::Instant] and sleeping
/// with [std::thread::sleep]
#[cfg(all(feature = "std", not(feature = "spin_sleep")))]
pub type Scheduler = CustomScheduler<std::time::Instant, StdSleeper>;

/// A scheduler representing instants with [std::time::Instant] and sleeping
/// with `spin_sleep::sleep`
#[cfg(all(feature = "std", feature = "spin_sleep"))]
pub type Scheduler = CustomScheduler<std::time::Instant, SpinSleeper>;

#[derive(Debug, Copy, Clone, Hash)]
pub struct CustomScheduler<INSTANT, SLEEPER> {
    target_delta: Duration,
    last_loop_end: INSTANT,
    sleeper: SLEEPER,
}

impl<INSTANT, SLEEPER> Default for CustomScheduler<INSTANT, SLEEPER>
where
    INSTANT: Instant,
    SLEEPER: Sleeper + Default,
{
    /// assumes a default hz of 60
    fn default() -> Self {
        CustomScheduler::new(60.0)
    }
}

impl<INSTANT, SLEEPER> CustomScheduler<INSTANT, SLEEPER>
where
    INSTANT: Instant,
    SLEEPER: Sleeper,
{
    /// Constructs a new Scheduler struct that can schedule tasks at the given frequency `target_hz`.
    pub fn new(target_hz: f32) -> Self
    where
        SLEEPER: Default,
    {
        Self::with_sleeper(target_hz, SLEEPER::default())
    }

    /// Constructs a new Scheduler struct that can schedule tasks at the given frequency `target_hz`, using a specific sleeper.
    pub fn with_sleeper(target_hz: f32, sleeper: SLEEPER) -> Self {
        let target_delta = Duration::from_secs_f32(target_hz.recip());
        CustomScheduler {
            target_delta,
            last_loop_end: INSTANT::now(),
            sleeper,
        }
    }

    /// Allows you to change the frequency at which the scheduler tries to run tasks.
    pub fn set_hz(&mut self, new_target_hz: f32) {
        self.target_delta = Duration::from_secs_f32(new_target_hz.recip())
    }

    /// Returns the frequency the Scheduler is currently set to.
    pub fn hz(&self) -> f32 {
        self.target_delta.as_secs_f32().recip()
    }

    /// Lets you run a task at a fixed interval, forever.
    /// ```rust, no_run
    /// # use spatial_led::{scheduler::Scheduler};
    /// pub fn main() {
    ///     let mut scheduler = Scheduler::new(120.0);
    ///     scheduler.loop_forever(|| {
    ///         println!("This will print 120 times per second!");
    ///     });
    /// }
    /// ```
    pub fn loop_forever(&mut self, mut task: impl FnMut()) -> ! {
        loop {
            task();
            self.sleep_until_next_frame();
        }
    }

    /// Lets you run a task at a fixed interval. Will break when the function returns false.
    /// ```rust
    /// # use spatial_led::{scheduler::Scheduler};
    /// pub fn main() {
    ///     let mut scheduler = Scheduler::new(240.0);
    ///     let mut loop_count = 0;
    ///     scheduler.loop_while_true(|| {
    ///         // do something
    ///         loop_count += 1;
    ///         return loop_count < 500;
    ///     });
    /// }
    /// ```
    pub fn loop_while_true(&mut self, mut task: impl FnMut() -> bool) {
        loop {
            if task() {
                break;
            }
            self.sleep_until_next_frame();
        }
    }

    /// Lets you run a task at a fixed interval. Will break when the function returns a result of Err variant.
    /// ```rust
    /// # use spatial_led::{scheduler::Scheduler};
    /// # use spatial_led::{Sled, SledResult};
    /// use palette::rgb::Rgb;
    /// pub fn main() {
    ///     let mut scheduler = Scheduler::new(60.0);
    ///     let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    ///     let mut segment_index = 0;
    ///     scheduler.loop_until_err(|| {
    ///         sled.set_segment(segment_index, Rgb::new(1.0, 1.0, 1.0))?;
    ///         segment_index += 1;
    ///         Ok(())
    ///     });
    /// }
    /// ```
    pub fn loop_until_err<T>(
        &mut self,
        mut task: impl FnMut() -> Result<T, Box<dyn core::error::Error>>,
    ) -> Box<dyn core::error::Error> {
        loop {
            match task() {
                Ok(_) => self.sleep_until_next_frame(),
                Err(e) => return e,
            }
        }
    }

    /// Can be called manually to sleep until the next scheduled frame.
    ///
    /// Valuable for when you'd like to avoid having to pass values into a closure, or would like more control over loop flow.
    /// ```rust
    /// # use spatial_led::{scheduler::Scheduler};
    /// pub fn main() {
    ///     let mut scheduler = Scheduler::new(60.0);
    ///
    ///     // print all numbers 0 to 59 in exactly one second.
    ///     for i in 0..60 {
    ///         println!("{}", i);
    ///         scheduler.sleep_until_next_frame();
    ///     }
    ///
    /// }
    /// ```
    pub fn sleep_until_next_frame(&mut self) {
        let elapsed = self.last_loop_end.elapsed();
        if elapsed < self.target_delta {
            self.sleeper.sleep(self.target_delta - elapsed);
            self.last_loop_end += self.target_delta;
        } else {
            self.last_loop_end = INSTANT::now();
        }
    }
}

#[derive(Debug, Copy, Clone, Hash)]
pub struct AsyncCustomScheduler<INSTANT, SLEEPER> {
    target_delta: Duration,
    last_loop_end: INSTANT,
    sleeper: SLEEPER,
}

impl<INSTANT, SLEEPER> Default for AsyncCustomScheduler<INSTANT, SLEEPER>
where
    INSTANT: Instant,
    SLEEPER: AsyncSleeper + Default,
{
    /// assumes a default hz of 60
    fn default() -> Self {
        Self::new(60.0)
    }
}

impl<INSTANT, SLEEPER> AsyncCustomScheduler<INSTANT, SLEEPER>
where
    INSTANT: Instant,
    SLEEPER: AsyncSleeper,
{
    /// Constructs a new AsyncCustomScheduler struct that can schedule tasks at the given frequency `target_hz`.
    pub fn new(target_hz: f32) -> Self
    where
        SLEEPER: Default,
    {
        Self::with_sleeper(target_hz, SLEEPER::default())
    }

    /// Constructs a new AsyncCustomScheduler struct that can schedule tasks at the given frequency `target_hz`.
    pub fn with_sleeper(target_hz: f32, sleeper: SLEEPER) -> Self {
        let target_delta = Duration::from_secs_f32(target_hz.recip());
        Self {
            target_delta,
            last_loop_end: INSTANT::now(),
            sleeper,
        }
    }

    /// Allows you to change the frequency at which the scheduler tries to run tasks.
    pub fn set_hz(&mut self, new_target_hz: f32) {
        self.target_delta = Duration::from_secs_f32(new_target_hz.recip())
    }

    /// Returns the frequency the AsyncCustomScheduler is currently set to.
    pub fn hz(&self) -> f32 {
        self.target_delta.as_secs_f32().recip()
    }

    /// Lets you run a task at a fixed interval, forever.
    pub async fn loop_forever(&mut self, mut task: impl FnMut()) -> ! {
        loop {
            task();
            self.sleep_until_next_frame().await;
        }
    }

    /// Lets you run a task at a fixed interval. Will break when the function returns false.
    pub async fn loop_while_true(&mut self, mut task: impl FnMut() -> bool) {
        loop {
            if task() {
                break;
            }
            self.sleep_until_next_frame().await;
        }
    }

    /// Lets you run a task at a fixed interval. Will break when the function returns a result of Err variant.
    pub async fn loop_until_err<T>(
        &mut self,
        mut task: impl FnMut() -> Result<T, Box<dyn core::error::Error>>,
    ) -> Box<dyn core::error::Error> {
        loop {
            match task() {
                Ok(_) => self.sleep_until_next_frame().await,
                Err(e) => return e,
            }
        }
    }

    /// Can be called manually to sleep until the next scheduled frame.
    ///
    /// Valuable for when you'd like to avoid having to pass values into a closure, or would like more control over loop flow.
    pub async fn sleep_until_next_frame(&mut self) {
        let elapsed = self.last_loop_end.elapsed();
        if elapsed < self.target_delta {
            self.sleeper.sleep(self.target_delta - elapsed).await;
            self.last_loop_end += self.target_delta;
        } else {
            self.last_loop_end = INSTANT::now();
        }
    }
}
