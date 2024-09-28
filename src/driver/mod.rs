use crate::{
    color::{Rgb, Srgb},
    Led, Sled, SledError, Vec2,
};

use std::time::{Duration, Instant};

mod filters;
// mod sliders;
mod buffers;
pub use buffers::BufferContainer;
pub use filters::Filters;

pub enum RefreshTiming {
    None,
    Fixed(f32),
}

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

pub struct Driver {
    sled: Option<Sled>,
    startup_commands: StartupCommands,
    compute_commands: ComputeCommands,
    draw_commands: DrawCommands,
    startup: Instant,
    last_update: Instant,
    buffers: BufferContainer,
    filters: Filters,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            sled: None,
            startup_commands: Box::new(|_, _, _| Ok(())),
            compute_commands: Box::new(|_, _, _, _| Ok(())),
            draw_commands: Box::new(|_, _, _, _| Ok(())),
            startup: Instant::now(),
            last_update: Instant::now(),
            buffers: BufferContainer::new(),
            filters: Filters::new(),
        }
    }

    pub fn sled(&self) -> Option<&Sled> {
        self.sled.as_ref()
    }

    pub fn elapsed(&self) -> Duration {
        self.startup.elapsed()
    }

    pub fn set_startup_commands<
        F: Fn(&mut Sled, &mut BufferContainer, &mut Filters) -> SledResult + 'static,
    >(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    pub fn set_compute_commands<
        F: Fn(&Sled, &mut BufferContainer, &mut Filters, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        compute_commands: F,
    ) {
        self.compute_commands = Box::new(compute_commands);
    }

    pub fn set_draw_commands<
        F: Fn(&mut Sled, &BufferContainer, &Filters, &TimeInfo) -> SledResult + 'static,
    >(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    pub fn mount(&mut self, mut sled: Sled) {
        (self.startup_commands)(&mut sled, &mut self.buffers, &mut self.filters).unwrap();
        self.startup = Instant::now();
        self.last_update = self.startup;
        self.sled = Some(sled);
    }

    pub fn step(&mut self) {
        if let Some(sled) = &mut self.sled {
            let time_info = TimeInfo {
                elapsed: self.startup.elapsed(),
                delta: self.last_update.elapsed(),
            };

            self.last_update = Instant::now();
            (self.compute_commands)(sled, &mut self.buffers, &mut self.filters, &time_info)
                .unwrap();
            (self.draw_commands)(sled, &self.buffers, &self.filters, &time_info).unwrap();
        }
    }

    pub fn step_by(&mut self, delta: Duration) {
        self.startup -= delta;
        self.step();
    }

    pub fn dismount(&mut self) -> Sled {
        self.sled.take().unwrap()
    }

    pub fn leds(&self) -> impl Iterator<Item = &Led> {
        if let Some(sled) = &self.sled {
            sled.leds()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

    pub fn colors(&self) -> impl Iterator<Item = &Rgb> + '_ {
        if let Some(sled) = &self.sled {
            sled.colors()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

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

    pub fn positions(&self) -> impl Iterator<Item = Vec2> + '_ {
        if let Some(sled) = &self.sled {
            sled.positions()
        } else {
            panic!("Driver has no Sled assigned!")
        }
    }

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

    pub fn buffers(&self) -> &BufferContainer {
        &self.buffers
    }

    pub fn buffers_mut(&mut self) -> &mut BufferContainer {
        &mut self.buffers
    }
}

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
}
