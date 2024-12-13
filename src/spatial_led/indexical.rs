use core::ops::Range;

use alloc::format;
use alloc::string::ToString;

use crate::{
    color::ColorType,
    error::SledError,
    led::Led,
    spatial_led::{Filter, Sled},
};

/// # Index-based read and write methods.
impl<COLOR: ColorType> Sled<COLOR> {
    /// Returns `Some(&Led<COLOR>)` if an [LED](Led) at `index` exists, `None` if not.
    ///
    /// O(1)
    pub fn get(&self, index: usize) -> Option<&Led<COLOR>> {
        self.leds.get(index)
    }

    /// Modulates the color of the [LED](Led) at `index` given a color rule function.
    /// Returns an [error](SledError) if no LED exists at that index.
    ///
    /// O(1)
    ///
    /// ```rust
    ///# use spatial_led::{Sled, SledError};
    ///# use palette::rgb::Rgb;
    ///# fn demo() -> Result<(), SledError> {
    ///# let mut sled = Sled::<Rgb>::new("./benches/config.yap")?;
    /// sled.modulate(0,
    ///     |led| led.color + Rgb::new(0.5, 0.0, 0.0)
    /// )?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate<F: Fn(&Led<COLOR>) -> COLOR>(
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

    /// Set the color of the [LED](Led) at `index` to `color`.
    /// Returns an [error](SledError) if no LED exists at that index.
    ///
    /// O(1)
    ///
    pub fn set(&mut self, index: usize, color: COLOR) -> Result<(), SledError> {
        if index >= self.num_leds {
            return SledError::new(format!("LED at index {} does not exist.", index)).as_err();
        }

        self.leds[index].color = color;
        Ok(())
    }

    /// Sets the color of all [LEDs](Led) in the system to `color`.
    ///
    /// O(LEDS)
    ///
    pub fn set_all(&mut self, color: COLOR) {
        for led in &mut self.leds {
            led.color = color;
        }
    }

    /// For each method that grants mutable access to each [LED](Led) in the system.
    ///
    /// O(LEDS)
    ///
    /// ```rust
    ///# use spatial_led::Sled;
    ///# use palette::rgb::Rgb;
    ///# let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.for_each(|led| {
    ///     if led.index() % 2 == 1 {
    ///         led.color = Rgb::new(1.0, 0.0, 0.0);
    ///     } else {
    ///         led.color = Rgb::new(0.0, 1.0, 0.0);
    ///     }
    /// });
    /// ```
    pub fn for_each<F: FnMut(&mut Led<COLOR>)>(&mut self, mut func: F) {
        for led in self.leds.iter_mut() {
            func(led);
        }
    }
}

/// # Index and range-based read and write methods
impl<COLOR : ColorType> Sled<COLOR> {
    /// Returns a Some([Filter]) containing all [LEDs](Led) with indices within `index_range`.
    /// Returns None if the range extends beyond the size of the system.
    ///
    /// O(RANGE_SIZE)
    ///
    pub fn range(&self, index_range: Range<usize>) -> Option<Filter> {
        if index_range.end < self.num_leds {
            let led_range = &self.leds[index_range];
            Some(led_range.into())
        } else {
            None
        }
    }

    /// Modulates the color of each [LED](Led) with indices in `index_range` given a color rule function.
    /// Returns an [error](SledError) if the range extends beyond the size of the system.
    ///
    /// O(RANGE_SIZE)
    ///
    /// ```rust
    ///# use spatial_led::{Sled, SledError};
    ///# use palette::rgb::Rgb;
    ///# fn main() -> Result<(), SledError> {
    ///# let mut sled = Sled::<Rgb>::new("./benches/config.yap")?;
    /// sled.modulate_range(0..50, |led| led.color * 0.5)?;
    ///# Ok(())
    ///# }
    /// ```
    pub fn modulate_range<F: Fn(&Led<COLOR>) -> COLOR>(
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

    /// Sets the color of the each [LED](Led) with indices in `index_range` to `color`.
    /// Returns an [error](SledError) if the range extends beyond the size of the system.
    ///
    /// O(RANGE_SIZE)
    ///
    pub fn set_range(&mut self, index_range: Range<usize>, color: COLOR) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return SledError::new("Index range extends beyond size of system.".to_string())
                .as_err();
        }

        self.leds[index_range]
            .iter_mut()
            .for_each(|led| led.color = color);
        Ok(())
    }

    /// For-each method granting mutable access to each [LED](Led) with an index in `index_range`
    ///
    /// Returns an [error](SledError) if the range extends beyond the size of the system.
    ///
    /// O(RANGE_SIZE)
    ///
    /// ```rust
    ///# use spatial_led::Sled;
    ///# use palette::rgb::Rgb;
    ///# let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.for_each_in_range(50..100, |led| {
    ///     if led.index() % 2 == 1 {
    ///         led.color = Rgb::new(1.0, 0.0, 0.0);
    ///     } else {
    ///         led.color = Rgb::new(0.0, 1.0, 0.0);
    ///     }
    /// });
    /// ```
    pub fn for_each_in_range<F: FnMut(&mut Led<COLOR>)>(
        &mut self,
        index_range: Range<usize>,
        func: F,
    ) -> Result<(), SledError> {
        if index_range.end >= self.num_leds {
            return SledError::new("Index range extends beyond size of system.".to_string())
                .as_err();
        }
        self.leds[index_range].iter_mut().for_each(func);
        Ok(())
    }
}
