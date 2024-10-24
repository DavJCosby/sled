use spin_sleep::SpinSleeper;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, Hash)]
pub struct Scheduler {
    target_delta: Duration,
    sleeper: SpinSleeper,
    last_loop_end: Instant,
}

impl Default for Scheduler {
    /// assumes a default hz of 60
    fn default() -> Self {
        Scheduler::new(60.0)
    }
}

impl Scheduler {
    /// Constructs a new Scheduler struct that can schedule tasks at the given frequency `target_hz`.
    pub fn new(target_hz: f32) -> Self {
        let target_delta = Duration::from_secs_f32(target_hz.recip());
        Scheduler {
            target_delta,
            sleeper: SpinSleeper::default(),
            last_loop_end: Instant::now(),
        }
    }

    /// Allows you to change the frequency at which the scheduler tries to run tasks.
    ///
    /// Note: Deprecated in favor of [Scheduler::set_hz()]
    #[deprecated]
    pub fn change_hz(&mut self, new_target_hz: f32) {
        self.target_delta = Duration::from_secs_f32(new_target_hz.recip())
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
    /// # use spatial_led::{Sled, SledResult, color::Rgb};
    /// pub fn main() {
    ///     let mut scheduler = Scheduler::new(60.0);
    ///     let mut sled = Sled::new("./examples/resources/config.yap").unwrap();
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
        mut task: impl FnMut() -> Result<T, Box<dyn std::error::Error>>,
    ) -> Box<dyn std::error::Error> {
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
            self.last_loop_end = Instant::now();
        }
    }
}
