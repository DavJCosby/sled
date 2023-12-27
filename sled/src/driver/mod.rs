use crate::{Filter, Sled, SledError};
use std::time::{Duration, Instant};

mod filters;
mod sliders;
mod scheduler;
pub use filters::Filters;
pub use sliders::{Slider, Sliders};

pub use scheduler::Scheduler;

pub enum RefreshTiming {
    None,
    Fixed(f32),
}

pub struct TimeInfo {
    pub elapsed: Duration,
    pub delta: Duration,
}

type SledResult = Result<(), SledError>;
type StartupCommands = Box<dyn Fn(&mut Sled, &mut Sliders, &mut Filters) -> SledResult>;
type DrawCommands = Box<dyn Fn(&mut Sled, &Sliders, &Filters, &TimeInfo) -> SledResult>;
pub struct Driver {
    _timing_strategy: RefreshTiming,
    sled: Option<Sled>,
    startup_commands: StartupCommands,
    draw_commands: DrawCommands,
    startup: Instant,
    last_update: Instant,
    sliders: Sliders,
    filters: Filters,
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            _timing_strategy: RefreshTiming::None,
            sled: None,
            startup_commands: Box::new(|_, _, _| Ok(())),
            draw_commands: Box::new(|_, _, _, _| Ok(())),
            startup: Instant::now(),
            last_update: Instant::now(),
            sliders: Sliders::new(),
            filters: Filters::new(),
        }
    }

    pub fn set_startup_commands<
        F: Fn(&mut Sled, &mut Sliders, &mut Filters) -> SledResult + 'static,
    >(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    pub fn set_draw_commands<
        F: Fn(&mut Sled, &Sliders, &Filters, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    pub fn mount(&mut self, mut sled: Sled) {
        (self.startup_commands)(&mut sled, &mut self.sliders, &mut self.filters).unwrap();
        self.startup = Instant::now();
        self.last_update = self.startup;
        self.sled = Some(sled);
    }

    pub fn update(&mut self) {
        if let Some(sled) = &mut self.sled {
            let time_info = TimeInfo {
                elapsed: self.startup.elapsed(),
                delta: self.last_update.elapsed(),
            };

            self.last_update = Instant::now();
            (self.draw_commands)(sled, &self.sliders, &self.filters, &time_info).unwrap();
        }
    }

    pub fn dismount(mut self) -> Sled {
        let sled = self.sled.unwrap();
        self.sled = None;
        sled
    }

    pub fn set_slider<T>(&mut self, key: &str, value: T)
    where
        Sliders: Slider<T>,
    {
        self.sliders.set(key, value)
    }

    pub fn get_slider<T: Copy>(&self, key: &str) -> Option<T>
    where
        Sliders: Slider<T>,
    {
        self.sliders.get(key)
    }

    pub fn insert_filter(&mut self, key: &str, set: Filter) {
        self.filters.set(key, set);
    }

    pub fn get_filter(&self, key: &str) -> Option<&Filter> {
        self.filters.get(key)
    }
}
