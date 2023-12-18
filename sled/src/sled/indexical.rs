use std::ops::{Index, IndexMut, Range};

use crate::{color::Rgb, error::SledError, led::Led, sled::Sled};

/// Index-based read and write methods.
impl Sled {
    pub fn get(&self, index: usize) -> Option<&Led> {
        self.leds.get(index)
    }

    pub fn modulate<F: Fn(&Led) -> Rgb>(
        &mut self,
        index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if index >= self.num_leds {
            return Err(SledError {
                message: format!("LED at index {} does not exist.", index),
            });
        }

        let led = &mut self.leds[index];
        led.color = color_rule(&led);
        Ok(())
    }

    pub fn set(&mut self, index: usize, color: Rgb) -> Result<(), SledError> {
        if index >= self.num_leds {
            return Err(SledError {
                message: format!("LED at index {} does not exist.", index),
            });
        }

        self.leds[index].color = color;
        Ok(())
    }

    pub fn set_all(&mut self, color: Rgb) {
        for led in &mut self.leds {
            led.color = color;
        }
    }

    pub fn for_each<F: FnMut(&mut Led)>(&mut self, mut func: F) {
        for led in self.leds.iter_mut() {
            func(led);
        }
    }
}

impl Index<usize> for Sled {
    type Output = Led;

    fn index(&self, index: usize) -> &Self::Output {
        &self.leds[index]
    }
}

impl IndexMut<usize> for Sled {
    fn index_mut(&mut self, index: usize) -> &mut Led {
        &mut self.leds[index]
    }
}

impl Index<Range<usize>> for Sled {
    type Output = [Led];

    fn index(&self, index_range: Range<usize>) -> &[Led] {
        &self.leds[index_range]
    }
}

impl IndexMut<Range<usize>> for Sled {
    fn index_mut(&mut self, index_range: Range<usize>) -> &mut [Led] {
        &mut self.leds[index_range]
    }
}

/// Index range-based read and write methods
impl Sled {
    pub fn get_range(&self, index_range: Range<usize>) -> Result<&[Led], SledError> {
        if index_range.end < self.num_leds {
            Ok(&self.leds[index_range])
        } else {
            Err(SledError {
                message: format!("Index range extends beyond size of system."),
            })
        }
    }

    pub fn modulate_range<F: Fn(&Led) -> Rgb>(
        &mut self,
        index_range: Range<usize>,
        color_rule: F,
    ) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return Err(SledError {
                message: format!("Index range extends beyond size of system."),
            });
        }

        for led in &mut self.leds[index_range] {
            led.color = color_rule(led);
        }

        Ok(())
    }

    pub fn set_range(&mut self, index_range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return Err(SledError {
                message: format!("Index range extends beyond size of system."),
            });
        }

        self.leds[index_range]
            .iter_mut()
            .for_each(|led| led.color = color);
        Ok(())
    }

    pub fn for_each_in_range<F: FnMut(&mut Led)>(
        &mut self,
        index_range: Range<usize>,
        mut func: F,
    ) {
        self.leds[index_range].iter_mut().for_each(|led| func(led));
    }
}
