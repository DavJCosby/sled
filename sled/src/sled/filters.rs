use std::collections::{hash_set, HashSet};

use crate::{led::Led, sled::Sled};
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
    pub fn from_vec(leds: Vec<&'f Led>) -> Self {
        let mut set = HashSet::new();
        for led in leds {
            set.insert(led);
        }
        Set { leds: set }
    }

    pub fn from_hashset(leds: HashSet<&'f Led>) -> Self {
        Set { leds }
    }

    fn filter(&self, filter: impl Fn(&Led) -> bool) -> Self {
        let filtered: HashSet<&'f Led> = self
            .leds
            .iter()
            .filter_map(|led| filter(led).then_some(*led))
            .collect();

        Set { leds: filtered }
    }

    fn and(&self, other: &Self) -> Self {
        let mut filtered = self.leds.clone();

        for led in &self.leds {
            if !other.leds.contains(led) {
                filtered.remove(led);
            }
        }

        Set { leds: filtered }
    }

    fn or(&self, other: &Self) -> Self {
        let mut extended = self.leds.clone();

        for led in &other.leds {
            extended.insert(*led);
        }

        Set { leds: extended }
    }

    fn get_closest(&self) -> &Led {
        self.leds
            .iter()
            .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }
    fn get_closest_to(&self, pos: Vec2) -> &Led {
        self.leds
            .iter()
            .min_by(|a, b| {
                let da = a.position().distance_squared(pos);
                let db = b.position().distance_squared(pos);
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    fn get_furthest(&self) -> &Led {
        self.leds
            .iter()
            .max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }
    fn get_furthest_from(&self, pos: Vec2) -> &Led {
        self.leds
            .iter()
            .max_by(|a, b| {
                let da = a.position().distance_squared(pos);
                let db = b.position().distance_squared(pos);
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    fn get_within_dist(&self, dist: f32) -> Self {
        self.filter(|led| led.distance() < dist)
    }
    fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Self {
        let dist_sq = dist.powi(2);
        self.filter(|led| led.position().distance_squared(pos) < dist_sq)
    }
}

/// Filters
impl Sled {
    pub fn filter(&self, filter: impl Fn(&Led) -> bool) -> Set {
        let filtered: HashSet<&Led> = self.leds.iter().filter(|led| filter(led)).collect();
        return filtered.into();
    }

    pub fn filter_by_angle(&self, angle_filter: impl Fn(f32) -> bool) -> Set {
        self.filter(|led| angle_filter(led.angle()))
    }

    pub fn filter_by_dir(&self, dir_filter: impl Fn(Vec2) -> bool) -> Set {
        self.filter(|led| dir_filter(led.direction()))
    }

    pub fn filter_by_pos(&self, pos_filter: impl Fn(Vec2) -> bool) -> Set {
        self.filter(|led| pos_filter(led.position()))
    }

    pub fn filter_by_dist(&self, dist_filter: impl Fn(f32) -> bool) -> Set {
        self.filter(|led| dist_filter(led.distance()))
    }

    pub fn filter_by_dist_from(&self, pos: Vec2, dist_filter: impl Fn(f32) -> bool) -> Set {
        self.filter(|led| {
            let dist = pos.distance(led.position());
            dist_filter(dist)
        })
    }
}

impl<'f> IntoIterator for Set<'f> {
    type Item = &'f Led;
    type IntoIter = hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.leds.into_iter()
    }
}
