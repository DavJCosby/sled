use alloc::collections::{btree_set, BTreeSet};

use crate::{color::ColorType, led::Led, spatial_led::Sled};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn len(&self) -> usize {
        self.led_indices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.led_indices.is_empty()
    }

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
    pub fn set_filter(&mut self, filter: &Filter, color: Color) {
        for i in filter {
            self.leds[i as usize].color = color;
        }
    }

    pub fn modulate_filter<F: Fn(&Led<Color>) -> Color>(&mut self, filter: &Filter, color_rule: F) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            led.color = color_rule(led)
        }
    }

    pub fn map_filter<F: Fn(&Led<Color>) -> Color>(&mut self, filter: &Filter, map: F) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            led.color = map(led)
        }
    }

    pub fn for_each_in_filter<F: FnMut(&mut Led<Color>)>(&mut self, filter: &Filter, mut func: F) {
        for i in filter {
            let led = &mut self.leds[i as usize];
            func(led);
        }
    }
}
