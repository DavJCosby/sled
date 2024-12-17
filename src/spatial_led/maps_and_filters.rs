use alloc::collections::BTreeSet;

#[cfg(not(feature = "std"))]
use num_traits::float::Float as _;

use crate::{
    color::ColorType,
    led::Led,
    spatial_led::{Filter, Sled},
    Vec2,
};

/// Maps
impl<Color: ColorType> Sled<Color> {
    /// Maps LEDs to a color.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map(|led| {
    ///     if led.direction().x > 0.0 {
    ///         Rgb::new(1.0, 0.0, 0.0)
    ///     } else {
    ///         Rgb::new(0.0, 0.0, 1.0)
    ///     }
    /// });
    /// ```
    pub fn map(&mut self, led_to_color_map: impl Fn(&Led<Color>) -> Color) {
        self.leds
            .iter_mut()
            .for_each(|led| led.color = led_to_color_map(led));
    }

    /// Maps LED indices to a color.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_index(|index| {
    ///     if index % 2 == 0 {
    ///         // even indices red
    ///         Rgb::new(1.0, 0.0, 0.0)
    ///     } else {
    ///         // odd indices blue
    ///         Rgb::new(0.0, 0.0, 1.0)
    ///     }
    /// });
    /// ```
    pub fn map_by_index(&mut self, index_to_color_map: impl Fn(usize) -> Color) {
        self.map(|led| index_to_color_map(led.index() as usize));
    }

    /// Maps LEDs to a color depending on which line segment they belong to.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_segment(|segment| {
    ///     match segment {
    ///         0..2 => Rgb::new(1.0, 0.0, 0.0),
    ///         2..4 => Rgb::new(0.0, 1.0, 0.0),
    ///         4..6 => Rgb::new(0.0, 0.0, 1.0),
    ///         _ => Rgb::new(0.0, 0.0, 0.0)
    ///     }
    /// });
    /// ```
    pub fn map_by_segment(&mut self, segment_index_to_color_map: impl Fn(usize) -> Color) {
        self.map(|led| segment_index_to_color_map(led.segment() as usize));
    }

    /// Maps LEDs positions to a color.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_pos(|pos| {
    ///     Rgb::new(
    ///         (pos.x.abs() / 10.0).min(1.0),
    ///         (pos.y.abs() / 10.0).min(1.0),
    ///         0.5
    ///     )
    /// });
    /// ```
    pub fn map_by_pos(&mut self, pos_to_color_map: impl Fn(Vec2) -> Color) {
        self.map(|led| pos_to_color_map(led.position()));
    }

    /// Maps LEDs directions (relative to the `center_point` defined in the config) to a color.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_dir(|dir| {
    ///     Rgb::new(
    ///         (dir.x / 2.0) + 0.5,
    ///         (dir.y / 2.0) + 0.5,
    ///         0.5
    ///     )
    /// });
    /// ```
    pub fn map_by_dir(&mut self, dir_to_color_map: impl Fn(Vec2) -> Color) {
        self.map(|led| dir_to_color_map(led.direction()));
    }

    /// Maps LEDs to a color depending on their direction from a given point.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_dir_from(Vec2::new(0.5, 1.5), |dir| {
    ///     Rgb::new(
    ///         (dir.x / 2.0) + 0.5,
    ///         (dir.y / 2.0) + 0.5,
    ///         0.5
    ///     )
    /// });
    /// ```
    pub fn map_by_dir_from(&mut self, point: Vec2, dir_to_color_map: impl Fn(Vec2) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let dir = (point - led.position()).normalize_or_zero();
            led.color = dir_to_color_map(dir)
        });
    }

    /// Maps LEDs angles (relative to the `center_point` defined in the config) to a color.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # use core::f32::consts::PI;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_angle(|angle| {
    ///     let brightness = angle / (PI * 2.0);
    ///     Rgb::new(brightness, brightness, brightness)
    /// });
    /// ```
    pub fn map_by_angle(&mut self, angle_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            led.color = angle_to_color_map(led.angle());
        });
    }

    /// Maps LEDs to a color depending on their angle from a given point.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # use core::f32::consts::PI;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_angle_from(Vec2::new(-1.0, -1.0), |angle| {
    ///     let brightness = angle / (PI * 2.0);
    ///     Rgb::new(brightness, brightness, brightness)
    /// });
    /// ```
    pub fn map_by_angle_from(&mut self, point: Vec2, angle_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let delta = point - led.position();
            let angle = delta.x.atan2(delta.y);
            led.color = angle_to_color_map(angle);
        });
    }

    /// Maps LEDs to a color depending on their distance from the `center_point`.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # use core::f32::consts::PI;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_dist(|dist| {
    ///     let transformed = (1.0 - (dist / 10.0)).max(0.0);
    ///     Rgb::new(transformed, 0.0, 0.0)
    /// });
    /// ```
    pub fn map_by_dist(&mut self, dist_to_color_map: impl Fn(f32) -> Color) {
        self.leds
            .iter_mut()
            .for_each(|led| led.color = dist_to_color_map(led.distance()));
    }

    /// Maps LEDs to a color depending on their distance from the given point.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # use core::f32::consts::PI;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// sled.map_by_dist_from(Vec2::new(2.0, -1.0), |dist| {
    ///     let transformed = (1.0 - (dist / 10.0)).max(0.0);
    ///     Rgb::new(transformed, 0.0, 0.0)
    /// });
    /// ```
    pub fn map_by_dist_from(&mut self, pos: Vec2, dist_to_color_map: impl Fn(f32) -> Color) {
        self.leds.iter_mut().for_each(|led| {
            let dist = pos.distance(led.position());
            led.color = dist_to_color_map(dist);
        });
    }
}

/// Filters
impl<Color: ColorType> Sled<Color> {
    /// Returns a [Filter] containing all LEDs that meet a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let odd = sled.filter(|led| led.index() % 2 == 0);
    /// sled.set_filter(&odd, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter(&self, filter: impl Fn(&Led<Color>) -> bool) -> Filter {
        let filtered: BTreeSet<u16> = self
            .leds
            .iter()
            .filter_map(|led| if filter(led) { Some(led.index()) } else { None })
            .collect();
        filtered.into()
    }

    /// Returns a [Filter] containing all LEDs whose angle meets a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let region = sled.filter_by_angle(|angle| angle > 0.0 && angle < 3.14);
    /// sled.set_filter(&region, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter_by_angle(&self, angle_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| angle_filter(led.angle()))
    }

    /// Returns a [Filter] containing all LEDs whose direction from the `center_point` meets a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let left = sled.filter_by_dir(|dir| dir.x < 0.0);
    /// sled.set_filter(&left, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter_by_dir(&self, dir_filter: impl Fn(Vec2) -> bool) -> Filter {
        self.filter(|led| dir_filter(led.direction()))
    }

    /// Returns a [Filter] containing all LEDs whose position meets a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let quadrants = sled.filter_by_pos(|pos| (pos.x * pos.y) > 0.0);
    /// sled.set_filter(&quadrants, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter_by_pos(&self, pos_filter: impl Fn(Vec2) -> bool) -> Filter {
        self.filter(|led| pos_filter(led.position()))
    }

    /// Returns a [Filter] containing all LEDs whose distance from the `center_point` meets a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let ring = sled.filter_by_dist(|dist| dist > 0.5 && dist < 0.75);
    /// sled.set_filter(&ring, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter_by_dist(&self, dist_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| dist_filter(led.distance()))
    }

    /// Returns a [Filter] containing all LEDs whose distance from the given point meets a certain criteria.
    ///
    /// ```rust
    /// # use spatial_led::{Sled, Filter, Vec2};
    /// # use palette::rgb::Rgb;
    /// # let mut sled = Sled::<Rgb>::new("./benches/config.yap").unwrap();
    /// let ring = sled.filter_by_dist_from(
    ///     Vec2::new(3.5, -0.25),
    ///     |dist| dist > 0.5 && dist < 0.75
    /// );
    /// sled.set_filter(&ring, Rgb::new(1.0, 1.0, 1.0));
    /// ```
    pub fn filter_by_dist_from(&self, pos: Vec2, dist_filter: impl Fn(f32) -> bool) -> Filter {
        self.filter(|led| {
            let dist = pos.distance(led.position());
            dist_filter(dist)
        })
    }
}
