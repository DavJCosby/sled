use std::collections::{hash_set, HashSet};

use crate::{color::Rgb, led::Led, sled::Sled};
use glam::Vec2;

pub struct Set<'f> {
    leds: HashSet<&'f Led>,
}

impl<'f> From<&'f [Led]> for Set<'f> {
    fn from(value: &'f [Led]) -> Self {
        let hs = HashSet::from_iter(value);
        return Set { leds: hs };
    }
}

impl<'f> From<HashSet<&'f Led>> for Set<'f> {
    fn from(value: HashSet<&'f Led>) -> Self {
        return Set { leds: value };
    }
}

#[allow(dead_code)]
impl<'f> Set<'f> {
    pub fn filter(&self, filter: impl Fn(&Led) -> bool) -> Self {
        let filtered: HashSet<&'f Led> = self
            .leds
            .iter()
            .filter_map(|led| filter(led).then_some(*led))
            .collect();

        Set { leds: filtered }
    }

    pub fn and(&self, other: &Self) -> Self {
        let mut filtered = self.leds.clone();

        for led in &self.leds {
            if !other.leds.contains(led) {
                filtered.remove(led);
            }
        }

        Set { leds: filtered }
    }

    pub fn or(&self, other: &Self) -> Self {
        let mut extended = self.leds.clone();

        for led in &other.leds {
            extended.insert(*led);
        }

        Set { leds: extended }
    }

    pub fn get_closest(&self) -> &Led {
        self.leds
            .iter()
            .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }

    pub fn get_closest_to(&self, pos: Vec2) -> &Led {
        self.leds
            .iter()
            .min_by(|a, b| {
                let da = a.position().distance_squared(pos);
                let db = b.position().distance_squared(pos);
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    pub fn get_furthest(&self) -> &Led {
        self.leds
            .iter()
            .max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }

    pub fn get_furthest_from(&self, pos: Vec2) -> &Led {
        self.leds
            .iter()
            .max_by(|a, b| {
                let da = a.position().distance_squared(pos);
                let db = b.position().distance_squared(pos);
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    pub fn get_within_dist(&self, dist: f32) -> Self {
        self.filter(|led| led.distance() < dist)
    }

    pub fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Self {
        let dist_sq = dist.powi(2);
        self.filter(|led| led.position().distance_squared(pos) < dist_sq)
    }
}

impl<'f> IntoIterator for Set<'f> {
    type Item = &'f Led;
    type IntoIter = hash_set::IntoIter<&'f Led>;

    fn into_iter(self) -> Self::IntoIter {
        self.leds.into_iter()
    }
}

impl<'s, 'f> IntoIterator for &'s Set<'f> {
    type Item = &'f Led;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // this doesn't seem right; revisit
        self.leds.clone().into_iter()
    }
}

impl Sled {
    pub fn set_leds_in_set(&mut self, set: &Set, color: Rgb) {
        for led in set {
            self.leds[led.index()].color = color;
        }
    }

    pub fn modulate_leds_in_set<F: Fn(&Led) -> Rgb>(&mut self, set: &Set, color_rule: F) {
        for led in set {
            self.leds[led.index()].color = color_rule(led)
        }
    }
}
