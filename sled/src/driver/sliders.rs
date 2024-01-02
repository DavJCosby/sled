use crate::Vec2;
use compact_str::{CompactString, ToCompactString};
use micromap::Map;

use self::internal_traits::SliderMapForType;
use crate::color::Rgb;

const MAP_DEPTH: usize = 10;
pub struct Sliders {
    colors: Map<CompactString, Rgb, MAP_DEPTH>,
    f32s: Map<CompactString, f32, MAP_DEPTH>,
    bools: Map<CompactString, bool, MAP_DEPTH>,
    vec2s: Map<CompactString, Vec2, MAP_DEPTH>,
    usizes: Map<CompactString, usize, MAP_DEPTH>,
}

#[allow(dead_code)]
impl Default for Sliders {
    fn default() -> Self {
        Self::new()
    }
}

impl Sliders {
    pub fn new() -> Self {
        Sliders {
            colors: Map::new(),
            f32s: Map::new(),
            bools: Map::new(),
            vec2s: Map::new(),
            usizes: Map::new(),
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        Sliders: SliderMapForType<T>,
    {
        let map = self.map_for_type_mut();
        if let Some(v) = map.get_mut(key) {
            *v = value
        } else {
            map.insert(key.to_compact_string(), value);
        }
    }

    pub fn get<T: Copy>(&self, key: &str) -> Option<T>
    where
        Sliders: SliderMapForType<T>,
    {
        let map = self.map_for_type();
        deref_option(map.get(key))
    }
}

mod internal_traits {
    use super::MAP_DEPTH;
    use compact_str::CompactString;
    use micromap::Map;

    pub trait SliderMapForType<T> {
        fn map_for_type(&self) -> &Map<CompactString, T, MAP_DEPTH>;
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, T, MAP_DEPTH>;
    }
}

impl SliderMapForType<Rgb> for Sliders {
    fn map_for_type(&self) -> &Map<CompactString, Rgb, MAP_DEPTH> {
        &self.colors
    }

    fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Rgb, MAP_DEPTH> {
        &mut self.colors
    }
}

impl SliderMapForType<f32> for Sliders {
    fn map_for_type(&self) -> &Map<CompactString, f32, MAP_DEPTH> {
        &self.f32s
    }

    fn map_for_type_mut(&mut self) -> &mut Map<CompactString, f32, MAP_DEPTH> {
        &mut self.f32s
    }
}

impl SliderMapForType<bool> for Sliders {
    fn map_for_type(&self) -> &Map<CompactString, bool, MAP_DEPTH> {
        &self.bools
    }

    fn map_for_type_mut(&mut self) -> &mut Map<CompactString, bool, MAP_DEPTH> {
        &mut self.bools
    }
}

impl SliderMapForType<Vec2> for Sliders {
    fn map_for_type(&self) -> &Map<CompactString, Vec2, MAP_DEPTH> {
        &self.vec2s
    }

    fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Vec2, MAP_DEPTH> {
        &mut self.vec2s
    }
}

impl SliderMapForType<usize> for Sliders {
    fn map_for_type(&self) -> &Map<CompactString, usize, MAP_DEPTH> {
        &self.usizes
    }

    fn map_for_type_mut(&mut self) -> &mut Map<CompactString, usize, MAP_DEPTH> {
        &mut self.usizes
    }
}

pub trait Slider<T>: SliderMapForType<T> {}
impl Slider<Rgb> for Sliders {}
impl Slider<f32> for Sliders {}
impl Slider<bool> for Sliders {}
impl Slider<Vec2> for Sliders {}
impl Slider<usize> for Sliders {}

fn deref_option<T: Copy>(option: Option<&T>) -> Option<T> {
    match option {
        Some(v) => Some(*v),
        None => None,
    }
}

