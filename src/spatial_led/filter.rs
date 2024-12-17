use alloc::collections::{btree_set, BTreeSet};

use crate::{color::ColorType, led::Led, spatial_led::Sled};

#[derive(Clone, Debug, PartialEq, Eq)]
/// A Filter is a set of LEDs that can be obtained via one of [Sled's](Sled) `.get()` methods.
///
/// They are particularly useful for scenarios when you have computationally expensive calculations and you want to limit which LEDs those calculations are run on. Once you've created a filter, you can save it to [Data](crate::driver::Data) for use in draw/compute stages. Using this pattern, we can pre-compute important sets at startup and then store them to the driver for later usage.
///
/// ```rust
/// # use spatial_led::{Sled, Filter, Vec2, driver::{Driver, Data, Time}};
/// # let mut driver = Driver::new();
///
/// driver.set_startup_commands(|sled, data| {
///     let area: Filter = sled.within_dist_from(
///         5.0, Vec2::new(-0.25, 1.5)
///     );
///
///    data.set("area_of_effect", area);
///     Ok(())
/// });
/// driver.set_draw_commands(|sled, data, _| {
///     let area_filter = data.get("area_of_effect")?;
///     sled.modulate_filter(area_filter, |led| {
///         // expensive computation
///     });
///     Ok(())
/// });
/// ```
pub struct Filter {
    led_indices: BTreeSet<u16>,
}

impl<Color: ColorType> From<&[Led<Color>]> for Filter {
    fn from(value: &[Led<Color>]) -> Self {
        let mut hs = BTreeSet::new();
        for led in value {
            hs.insert(led.index());
        }
        Filter { led_indices: hs }
    }
}

impl From<BTreeSet<u16>> for Filter {
    fn from(value: BTreeSet<u16>) -> Self {
        Filter { led_indices: value }
    }
}

#[allow(dead_code)]
impl Filter {
    /// Returns the number of leds contained in the underlying set.
    pub fn len(&self) -> usize {
        self.led_indices.len()
    }

    /// Returns true if the underlying set is empty.
    pub fn is_empty(&self) -> bool {
        self.led_indices.is_empty()
    }

    /// Returns a new Filter containing all lEDs that were in both this and the inputted other Filter.
    pub fn and(&self, other: &Self) -> Self {
        let mut filtered = self.led_indices.clone();
        for led in &self.led_indices {
            if !other.led_indices.contains(led) {
                filtered.remove(led);
            }
        }

        Filter {
            led_indices: filtered,
        }
    }

    /// Returns a new Filter containing all lEDs that were in either this or the inputted other Filter.
    pub fn or(&self, other: &Self) -> Self {
        let mut extended = self.led_indices.clone();

        for led in &other.led_indices {
            extended.insert(*led);
        }

        Filter {
            led_indices: extended,
        }
    }
}

impl IntoIterator for Filter {
    type Item = u16;
    type IntoIter = btree_set::IntoIter<u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.led_indices.into_iter()
    }
}

impl IntoIterator for &Filter {
    type Item = u16;
    type IntoIter = btree_set::IntoIter<u16>;

    fn into_iter(self) -> Self::IntoIter {
        // this doesn't seem right; revisit
        self.led_indices.clone().into_iter()
    }
}

impl FromIterator<u16> for Filter {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let mut set = BTreeSet::<u16>::new();
        for i in iter {
            set.insert(i);
        }

        Filter { led_indices: set }
    }
}

impl Extend<u16> for Filter {
    fn extend<T: IntoIterator<Item = u16>>(&mut self, iter: T) {
        for i in iter {
            self.led_indices.insert(i);
        }
    }
}

impl<Color: ColorType> Sled<Color> {
    /// Sets all LEDs in the given filter to `color`.
    ///
    /// O(LEDS_IN_FILTER)
    pub fn set_filter(&mut self, filter: &Filter, color: Color) {
        for i in filter {
            self.leds[i as usize].color = color;
        }
    }

    /// Modulates the color of each LED contained in the filter.
    ///
    /// O(LEDS_IN_FILTER)
    /// ```rust
    /// # use spatial_led::{Sled, SledError, Vec2};
    /// # use palette::rgb::Rgb;
    /// # fn demo() -> Result<(), SledError> {
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap")?;
    /// let first_wall = sled.segment(0).unwrap();
    /// let third_wall = sled.segment(2).unwrap();
    /// let first_and_third = first_wall.or(&third_wall);
    /// // dim first and third walls by 50%
    /// sled.modulate_filter(&first_and_third, |led| led.color * 0.5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn modulate_filter<F: Fn(&Led<Color>) -> Color>(&mut self, filter: &Filter, color_rule: F) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            led.color = color_rule(led)
        }
    }

    /// Functionally identical to `modulate_filter()`.
    pub fn map_filter(&mut self, filter: &Filter, color_map: impl Fn(&Led<Color>) -> Color) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            led.color = color_map(led)
        }
    }

    /// For-each method granting mutable access to each LED contained in the given filter.
    ///
    /// O(LEDS_IN_FILTER)
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let circle: Filter = sled.within_dist(2.5);
    /// sled.for_each_in_filter(&circle, |led| {
    ///     if led.position().x > 0.0 {
    ///         led.color = Rgb::new(1.0, 0.0, 0.0);
    ///     } else {
    ///         led.color = Rgb::new(0.0, 0.0, 1.0);
    ///     }
    /// });
    /// ```
    pub fn for_each_in_filter<F: FnMut(&mut Led<Color>)>(&mut self, filter: &Filter, mut func: F) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            func(led);
        }
    }
}
