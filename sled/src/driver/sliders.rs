use std::collections::HashMap;

use self::internal::SliderMapForType;
use crate::color::Rgb;

pub struct Sliders {
    color_sliders: HashMap<String, Rgb>,
    f32_sliders: HashMap<String, f32>,
}

#[allow(dead_code)]
impl Sliders {
    pub fn new() -> Self {
        Sliders {
            color_sliders: HashMap::new(),
            f32_sliders: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        Sliders: internal::SliderMapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.insert(key.to_string(), value);
    }

    pub fn get<T>(&self, key: &str) -> Option<&T>
    where
        Sliders: internal::SliderMapForType<T>,
    {
        let map = self.map_for_type();
        return map.get(key);
    }
}

pub trait Slider<T>: internal::SliderMapForType<T> {}
impl Slider<Rgb> for Sliders {}
impl Slider<f32> for Sliders {}

mod internal {
    use std::collections::HashMap;

    pub trait SliderMapForType<T> {
        fn map_for_type(&self) -> &HashMap<String, T>;
        fn map_for_type_mut(&mut self) -> &mut HashMap<String, T>;
    }
}

impl internal::SliderMapForType<Rgb> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, Rgb> {
        &self.color_sliders
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, Rgb> {
        &mut self.color_sliders
    }
}

impl internal::SliderMapForType<f32> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, f32> {
        &self.f32_sliders
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, f32> {
        &mut self.f32_sliders
    }
}
