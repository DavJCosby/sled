use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{color::Rgb, Sled, SledError};

pub enum RefreshTiming {
    None,
    Fixed(f32),
}

pub struct TimeInfo {
    pub elapsed: Duration,
    pub delta: Duration,
}

pub trait Slider: internal::Slider {}

mod internal {
    pub trait Slider {
        fn get_slider_of_this_type<'a>(
            sliders: &'a super::Sliders,
            key: &'a str,
        ) -> Option<&'a Self>
        where
            Self: Sized;

        fn set_slider_of_this_type(&self, sliders: &mut super::Sliders, key: &str);
    }
}

impl Slider for Rgb {}
impl Slider for f32 {}

impl internal::Slider for Rgb {
    fn get_slider_of_this_type<'a>(sliders: &'a Sliders, key: &'a str) -> Option<&'a Self>
    where
        Self: Sized,
    {
        sliders.color_sliders.get(key)
    }

    fn set_slider_of_this_type(&self, sliders: &mut Sliders, key: &str) {
        sliders.color_sliders.insert(key.to_string(), *self);
    }
}

impl internal::Slider for f32 {
    fn get_slider_of_this_type<'a>(sliders: &'a Sliders, key: &'a str) -> Option<&'a Self>
    where
        Self: Sized,
    {
        sliders.f32_sliders.get(key)
    }

    fn set_slider_of_this_type(&self, sliders: &mut Sliders, key: &str) {
        sliders.f32_sliders.insert(key.to_string(), *self);
    }
}

// A polymorphic solution that is both clever and vile.
pub struct Sliders {
    color_sliders: HashMap<String, Rgb>,
    f32_sliders: HashMap<String, f32>,
}

impl Sliders {
    pub fn new() -> Self {
        Sliders {
            color_sliders: HashMap::new(),
            f32_sliders: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: impl Slider) {
        // I know how it looks, but hear me out...
        value.set_slider_of_this_type(self, key)
    }

    pub fn get<'a, T: Slider>(&'a self, key: &'a str) -> Option<&T> {
        // This might be even worse lol
        T::get_slider_of_this_type(&self, key)
    }
}

pub struct Driver {
    timing_strategy: RefreshTiming,
    sled: Option<Sled>,
    startup_commands: Box<dyn Fn(&mut Sled) -> Result<(), SledError>>,
    draw_commands: Box<dyn Fn(&mut Sled, &TimeInfo) -> Result<(), SledError>>,
    startup: Instant,
    last_update: Instant,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            timing_strategy: RefreshTiming::None,
            sled: None,
            startup_commands: Box::new(|_| Ok(())),
            draw_commands: Box::new(|_, _| Ok(())),
            startup: Instant::now(),
            last_update: Instant::now(),
        }
    }

    pub fn set_startup_commands<F: Fn(&mut Sled) -> Result<(), SledError> + 'static>(
        &mut self,
        startup_commands: F,
    ) {
        self.startup_commands = Box::new(startup_commands);
    }

    pub fn set_draw_commands<F: Fn(&mut Sled, &TimeInfo) -> Result<(), SledError> + 'static>(
        &mut self,
        draw_commands: F,
    ) {
        self.draw_commands = Box::new(draw_commands);
    }

    pub fn mount(&mut self, mut sled: Sled) {
        (self.startup_commands)(&mut sled).unwrap();

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
            (self.draw_commands)(sled, &time_info).unwrap();
        }
    }
}
