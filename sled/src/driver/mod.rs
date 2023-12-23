use crate::{Sled, SledError};
use std::time::{Duration, Instant};

mod sliders;
use sliders::{Slider, Sliders};

pub enum RefreshTiming {
    None,
    Fixed(f32),
}

pub struct TimeInfo {
    pub elapsed: Duration,
    pub delta: Duration,
}

type SledResult = Result<(), SledError>;
type StartupCommands = Box<dyn Fn(&mut Sled, &mut Sliders) -> SledResult>;
type DrawCommands = Box<dyn Fn(&mut Sled, &Sliders, &TimeInfo) -> SledResult>;
pub struct Driver {
    _timing_strategy: RefreshTiming,
    sled: Option<Sled>,
    startup_commands: StartupCommands,
    draw_commands: DrawCommands,
    startup: Instant,
    last_update: Instant,
    sliders: Sliders,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            _timing_strategy: RefreshTiming::None,
            sled: None,
            startup_commands: Box::new(|_, _| Ok(())),
            draw_commands: Box::new(|_, _, _| Ok(())),
            startup: Instant::now(),
            last_update: Instant::now(),
            sliders: Sliders::new(),
        }
    }

    pub fn set_startup_commands<F: Fn(&mut Sled, &mut Sliders) -> SledResult + 'static>(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    pub fn set_draw_commands<F: Fn(&mut Sled, &Sliders, &TimeInfo) -> SledResult + 'static>(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    pub fn mount(&mut self, mut sled: Sled) {
        (self.startup_commands)(&mut sled, &mut self.sliders).unwrap();

        self.sled = Some(sled);
        self.startup = Instant::now();
        self.last_update = self.startup.clone();
    }

    pub fn update(&mut self) {
        if let Some(sled) = &mut self.sled {
            let time_info = TimeInfo {
                elapsed: self.startup.elapsed(),
                delta: self.last_update.elapsed(),
            };

            self.last_update = Instant::now();
            (self.draw_commands)(sled, &self.sliders, &time_info).unwrap();
        }
    }

    pub fn set_slider<T>(&mut self, key: &str, value: T)
    where
        Sliders: Slider<T>,
    {
        self.sliders.set(key, value)
    }

    pub fn get_slider<T>(&self, key: &str) -> Option<&T>
    where
        Sliders: Slider<T>,
    {
        self.sliders.get(key)
    }
}
