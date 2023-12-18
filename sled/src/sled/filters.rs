use std::collections::HashSet;

use crate::{led::Led, sled::Sled};
use glam::Vec2;

/// Filters
impl Sled {
    pub fn filter(&self, filter: impl Fn(&Led) -> bool) -> Vec<&Led> {
        return self.leds.iter().filter(|led| filter(led)).collect();
    }

    // pub fn filter_mut(&mut self, filter: impl Fn(&Led) -> bool) -> Vec<&mut Led> {
    //     return self.leds.iter_mut().filter(|led| filter(led)).collect();
    // }

    pub fn filter_by_angle(&self, angle_filter: impl Fn(f32) -> bool) -> Vec<&Led> {
        self.filter(|led| angle_filter(led.angle()))
    }

    // pub fn filter_by_angle_mut(&mut self, angle_filter: impl Fn(f32) -> bool) -> Vec<&mut Led> {
    //     self.filter_mut(|led| angle_filter(led.angle()))
    // }

    pub fn filter_by_dir(&self, dir_filter: impl Fn(Vec2) -> bool) -> Vec<&Led> {
        self.filter(|led| dir_filter(led.direction()))
    }

    // pub fn filter_by_dir_mut(&mut self, dir_filter: impl Fn(Vec2) -> bool) -> Vec<&mut Led> {
    //     self.filter_mut(|led| dir_filter(led.direction()))
    // }

    pub fn filter_by_pos(&self, pos_filter: impl Fn(Vec2) -> bool) -> Vec<&Led> {
        self.filter(|led| pos_filter(led.position()))
    }

    // pub fn filter_by_pos_mut(&mut self, pos_filter: impl Fn(Vec2) -> bool) -> Vec<&mut Led> {
    //     self.filter_mut(|led| pos_filter(led.position()))
    // }

    pub fn filter_by_dist(&self, dist_filter: impl Fn(f32) -> bool) -> Vec<&Led> {
        self.filter(|led| dist_filter(led.distance()))
    }

    // pub fn filter_by_dist_mut(&mut self, dist_filter: impl Fn(f32) -> bool) -> Vec<&mut Led> {
    //     self.filter_mut(|led| dist_filter(led.distance()))
    // }

    pub fn filter_by_dist_from(&self, pos: Vec2, dist_filter: impl Fn(f32) -> bool) -> Vec<&Led> {
        self.filter(|led| {
            let dist = pos.distance(led.position());
            dist_filter(dist)
        })
    }

    // pub fn filter_by_dist_from_mut(
    //     &mut self,
    //     pos: Vec2,
    //     dist_filter: impl Fn(f32) -> bool,
    // ) -> Vec<&mut Led> {
    //     self.filter_mut(|led| {
    //         let dist = pos.distance(led.position());
    //         dist_filter(dist)
    //     })
    // }
}

pub trait CollectionOfLeds {
    // syntax sugar for .iter().filter().collect()
    fn filter(&self, filter: impl Fn(&Led) -> bool) -> Self;

    fn and(&mut self, other: &Self) -> Self;
    fn or(&mut self, other: &Self) -> Self;

    fn get_closest(&self) -> &Led;
    fn get_closest_to(&self, pos: Vec2) -> &Led;

    fn get_furthest(&self) -> &Led;
    fn get_furthest_from(&self, pos: Vec2) -> &Led;

    fn get_within_dist(&self, dist: f32) -> Self;
    fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Self;
}

impl CollectionOfLeds for Vec<&Led> {
    fn filter(&self, filter: impl Fn(&Led) -> bool) -> Self {
        self.iter()
            .filter_map(|led| filter(led).then_some(*led))
            .collect()
    }

    fn and(&mut self, other: &Self) -> Self {
        // definitely can be optimized
        let mut copy = self.clone();
        copy.retain(|led| other.contains(led));
        copy
    }

    fn or(&mut self, other: &Self) -> Self {
        let mut set = HashSet::new();
        self.iter().for_each(|led| {
            set.insert(*led);
        });

        other.iter().for_each(|led| {
            set.insert(*led);
        });

        set.iter().map(|led| *led).collect()
    }

    fn get_closest(&self) -> &Led {
        self.iter()
            .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }

    fn get_closest_to(&self, pos: Vec2) -> &Led {
        self.iter()
            .min_by(|a, b| {
                let da = a.position().distance_squared(pos);
                let db = b.position().distance_squared(pos);
                da.partial_cmp(&db).unwrap()
            })
            .unwrap()
    }

    fn get_furthest(&self) -> &Led {
        self.iter()
            .max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
            .unwrap()
    }

    fn get_furthest_from(&self, pos: Vec2) -> &Led {
        self.iter()
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

// pub trait CollectionOfLedsMut {
//     /// consumes other and modifies itself
//     fn and(&mut self, other: Self);
//     /// consumes other and modifies itself
//     fn or(&mut self, other: Self);

//     fn get_closest(&self) -> &Led;
//     fn get_closest_to(&self, pos: Vec2) -> &Led;
//     fn set_closest(&mut self, color: Rgb);
//     fn set_closest_to(&mut self, pos: Vec2, color: Rgb);
//     fn modulate_closest(&mut self, color_rule: )

//     fn get_furthest(&self) -> &Led;
//     fn get_furthest_from(&self, pos: Vec2) -> &Led;

//     fn get_within_dist(&self, dist: f32) -> Self;
//     fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Self;

//     fn set_all(&mut self, color: Rgb);
//     fn set_closest_to(&mut self, pos: Vec2, color: Rgb);

//     fn map(&mut self, led_to_color_map: impl Fn(&Led) -> Rgb);
// }

// impl CollectionOfLedsMut for Vec<&mut Led> {
//     fn and(&mut self, other: Self) {
//         self.retain(|led| other.contains(led));
//     }

//     fn or(&mut self, other: Self) {
//         for led in other {
//             if !self.contains(&led) {
//                 self.push(led);
//             }
//         }
//     }

//     fn get_closest(&self) -> &Led {
//         self.iter()
//             .min_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
//             .unwrap()
//     }

//     fn get_closest_to(&self, pos: Vec2) -> &Led {
//         self.iter()
//             .min_by(|a, b| {
//                 let da = a.position().distance_squared(pos);
//                 let db = b.position().distance_squared(pos);
//                 da.partial_cmp(&db).unwrap()
//             })
//             .unwrap()
//     }

//     fn get_furthest(&self) -> &Led {
//         self.iter()
//             .max_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap())
//             .unwrap()
//     }

//     fn get_furthest_from(&self, pos: Vec2) -> &Led {
//         self.iter()
//             .max_by(|a, b| {
//                 let da = a.position().distance_squared(pos);
//                 let db = b.position().distance_squared(pos);
//                 da.partial_cmp(&db).unwrap()
//             })
//             .unwrap()
//     }

//     fn get_within_dist(&self, dist: f32) -> Self {
//         self.iter()
//             .filter_map(|led| (led.distance() < dist).then_some(*led))
//             .collect()
//     }

//     fn get_within_dist_from(&self, dist: f32, pos: Vec2) -> Self {
//         let dist_sq = dist.powi(2);
//         self.iter()
//             .filter_map(|led| (led.position().distance_squared(pos) < dist_sq).then_some(*led))
//             .collect()
//     }

//     fn set_all(&mut self, color: Rgb) {
//         for led in self {
//             led.color = color;
//         }
//     }

//     fn set_closest_to(&mut self, pos: Vec2, color: Rgb) {
//         todo!()
//     }

//     fn map(&mut self, led_to_color_map: impl Fn(&Led) -> Rgb) {
//         todo!()
//     }
// }
