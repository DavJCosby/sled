use std::ops::Range;

use crate::{
    color::Rgb,
    error::SledError,
    led::Led,
    sled::{Filter, Sled},
};

/// Index-based read and write methods.
impl Sled {
    /// Returns `Some(&Led)` if an Led at `index` exists, `None` if not.
    pub fn get(&self, index: usize) -> Option<&Led> {
        self.leds.get(index)
    }

    /// Modulates the color of the led at `index` given a color rule function.
    /// Returns an error if no led exists at that index.
    /// ```rust
    ///# use sled::{Sled, SledError, color::Rgb};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// sled.modulate(0,
    ///     |led| led.color + Rgb::new(0.5, 0.0, 0.0)
    /// )?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate<F: Fn(&Led) -> Rgb>(
        &mut self,
        index: usize,
        color_rule: F,
    ) -> Result<(), SledError> {
        if index >= self.num_leds {
            return SledError::new(format!("LED at index {} does not exist.", index)).as_err();
        }

        let led = &mut self.leds[index];
        led.color = color_rule(led);
        Ok(())
    }

    /// Set the color of the LED at `index` to `color`.
    /// Returns an error if no led exists at that index.
    pub fn set(&mut self, index: usize, color: Rgb) -> Result<(), SledError> {
        if index >= self.num_leds {
            return SledError::new(format!("LED at index {} does not exist.", index)).as_err();
        }

        self.leds[index].color = color;
        Ok(())
    }

    /// Sets the color of all LEDs in the system to `color`.
    pub fn set_all(&mut self, color: Rgb) {
        for led in &mut self.leds {
            led.color = color;
        }
    }

    /// For each method granting mutable access to each LED in the system.
    /// ```rust
    ///# use sled::{Sled, color::Rgb};
    ///# let mut sled = Sled::new("./examples/resources/config.toml").unwrap();
    /// sled.for_each(|led| {
    ///     if led.index() % 2 == 1 {
    ///         led.color = Rgb::new(1.0, 0.0, 0.0);
    ///     } else {
    ///         led.color = Rgb::new(0.0, 1.0, 0.0);
    ///     }
    /// });
    /// ```
    pub fn for_each<F: FnMut(&mut Led)>(&mut self, mut func: F) {
        for led in self.leds.iter_mut() {
            func(led);
        }
    }
}

/// Index range-based read and write methods
impl Sled {
    /// Returns a [Filter] containing all LEDs with indices within `index_range`.
    /// Returns an error if the range extends beyond the size of the system.
    pub fn get_range(&self, index_range: Range<usize>) -> Result<Filter, SledError> {
        if index_range.end < self.num_leds {
            let led_range = &self.leds[index_range];
            Ok(led_range.into())
        } else {
            SledError::new("Index range extends beyond size of system.".to_string()).as_err()
        }
    }

    /// Modulates the color of the each LED with indices in `index_range` given a color rule function.
    /// Returns an error if the range extends beyond the size of the system.
    /// ```rust
    ///# use sled::{Sled, SledError};
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::new("./examples/resources/config.toml")?;
    /// sled.modulate_range(0..50, |led| led.color * 0.5)?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_range<F: Fn(&Led) -> Rgb>(
        &mut self,
        index_range: Range<usize>,
        color_rule: F,
    ) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return SledError::new("Index range extends beyond size of system.".to_string())
                .as_err();
        }

        for led in &mut self.leds[index_range] {
            led.color = color_rule(led);
        }

        Ok(())
    }

    /// Sets the color of the each LED with indices in `index_range` to `color`.
    /// Returns an error if the range extends beyond the size of the system.
    pub fn set_range(&mut self, index_range: Range<usize>, color: Rgb) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return SledError::new("Index range extends beyond size of system.".to_string())
                .as_err();
        }

        self.leds[index_range]
            .iter_mut()
            .for_each(|led| led.color = color);
        Ok(())
    }

    /// For each method granting mutable access to each LED with an index in `index_range`
    /// ```rust
    ///# use sled::{Sled, color::Rgb};
    ///# let mut sled = Sled::new("./examples/resources/config.toml").unwrap();
    /// sled.for_each_in_range(50..100, |led| {
    ///     if led.index() % 2 == 1 {
    ///         led.color = Rgb::new(1.0, 0.0, 0.0);
    ///     } else {
    ///         led.color = Rgb::new(0.0, 1.0, 0.0);
    ///     }
    /// });
    /// ```
    pub fn for_each_in_range<F: FnMut(&mut Led)>(&mut self, index_range: Range<usize>, func: F) {
        self.leds[index_range].iter_mut().for_each(func);
    }
}