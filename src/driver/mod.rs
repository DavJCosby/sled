use crate::{color::Srgb, Sled, SledError, Vec2};
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

impl Default for Driver {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn dismount(mut self) -> Sled {
        let sled = self.sled.unwrap();
        self.sled = None;
        sled
    }

    pub fn colors<T>(&self) -> Vec<Srgb<T>>
    where
        f32: crate::color::stimulus::IntoStimulus<T>,
    {
        if let Some(sled) = &self.sled {
            sled.colors()
        } else {
            vec![]
        }
    }

    pub fn positions(&self) -> Vec<Vec2> {
        if let Some(sled) = &self.sled {
            sled.positions()
        } else {
            vec![]
        }
    }

    pub fn colors_and_positions<T>(&self) -> Vec<(Srgb<T>, Vec2)>
    where
        f32: crate::color::stimulus::IntoStimulus<T>,
    {
        if let Some(sled) = &self.sled {
            sled.colors_and_positions()
        } else {
            vec![]
        }
    }

    pub fn buffers(&self) -> &BufferContainer {
        &self.buffers
    }

    pub fn buffers_mut(&mut self) -> &mut BufferContainer {
        &mut self.buffers
    }
}
