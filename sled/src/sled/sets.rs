use std::collections::{hash_set, HashSet};

use crate::{color::Rgb, led::Led, sled::Sled};

#[derive(Clone)]
pub struct Set {
    led_indices: HashSet<usize>,
}

impl From<&[Led]> for Set {
    fn from(value: &[Led]) -> Self {
        let mut hs = HashSet::new();
        for led in value {
            hs.insert(led.index());
        }
        return Set { led_indices: hs };
    }
}

impl From<HashSet<usize>> for Set {
    fn from(value: HashSet<usize>) -> Self {
        return Set { led_indices: value };
    }
}

#[allow(dead_code)]
impl Set {
    pub fn len(&self) -> usize {
        return self.led_indices.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.led_indices.is_empty();
    }

    pub fn and(&self, other: &Self) -> Self {
        let mut filtered = self.led_indices.clone();

        for led in &self.led_indices {
            if !other.led_indices.contains(led) {
                filtered.remove(led);
            }
        }

        Set {
            led_indices: filtered,
        }
    }

    pub fn or(&self, other: &Self) -> Self {
        let mut extended = self.led_indices.clone();

        for led in &other.led_indices {
            extended.insert(*led);
        }

        Set {
            led_indices: extended,
        }
    }
}

impl IntoIterator for Set {
    type Item = usize;
    type IntoIter = hash_set::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.led_indices.into_iter()
    }
}

impl IntoIterator for &Set {
    type Item = usize;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // this doesn't seem right; revisit
        self.led_indices.clone().into_iter()
    }
}

impl Sled {
    pub fn set_leds_in_set(&mut self, set: &Set, color: Rgb) {
        for i in set {
            self.leds[i].color = color;
        }
    }

    pub fn modulate_leds_in_set<F: Fn(&Led) -> Rgb>(&mut self, set: &Set, color_rule: F) {
        for i in set {
            let led = &mut self.leds[i];
            led.color = color_rule(led)
        }
    }
}
