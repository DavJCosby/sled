use crate::{Set, Sled, SledError};
use std::time::{Duration, Instant};

mod sets;
mod sliders;
use sets::Sets;
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
type StartupCommands = Box<dyn Fn(&mut Sled, &mut Sliders, &mut Sets) -> SledResult>;
type DrawCommands = Box<dyn Fn(&mut Sled, &Sliders, &Sets, &TimeInfo) -> SledResult>;
pub struct Driver {
    _timing_strategy: RefreshTiming,
    sled: Option<Sled>,
    startup_commands: StartupCommands,
    draw_commands: DrawCommands,
    startup: Instant,
    last_update: Instant,
    sliders: Sliders,
    sets: Sets,
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
            sets: Sets::new(),
        }
    }

    pub fn set_startup_commands<
        F: Fn(&mut Sled, &mut Sliders, &mut Sets) -> SledResult + 'static,
    >(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    pub fn set_draw_commands<
        F: Fn(&mut Sled, &Sliders, &Sets, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    pub fn mount(&mut self, sled: Sled) {
        self.sled = Some(sled);
        self.startup();
    }

    fn startup(&mut self) {
        if let Some(sled) = &mut self.sled {
            (self.startup_commands)(sled, &mut self.sliders, &mut self.sets).unwrap();
            self.startup = Instant::now();
            self.last_update = self.startup;
        }
    }

    pub fn update(&mut self) {
        if self.sled.is_none() {
            return;
        }

        let time_info = TimeInfo {
            elapsed: self.startup.elapsed(),
            delta: self.last_update.elapsed(),
        };

        self.last_update = Instant::now();
        (self.draw_commands)(
            self.sled.as_mut().unwrap(),
            &self.sliders,
            &self.sets,
            &time_info,
        )
        .unwrap();
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

    pub fn insert_set(&mut self, key: &str, set: Set) {
        self.sets.set(key, set);
    }

    pub fn get_set(&self, key: &str) -> Option<&Set> {
        self.sets.get(key)
    }
}
