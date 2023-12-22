use std::collections::HashMap;

use glam::Vec3;

use self::internal_traits::SliderMapForType;
use crate::color::Rgb;

pub struct Sliders {
    colors: HashMap<String, Rgb>,
    f32s: HashMap<String, f32>,
    bools: HashMap<String, bool>,
    vec3s: HashMap<String, Vec3>,
    usizes: HashMap<String, usize>,
}

#[allow(dead_code)]
impl Sliders {
    pub fn new() -> Self {
        Sliders {
            colors: HashMap::new(),
            f32s: HashMap::new(),
            bools: HashMap::new(),
            vec3s: HashMap::new(),
            usizes: HashMap::new(),
        }
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        Sliders: SliderMapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.insert(key.to_string(), value);
    }

    pub fn get<T>(&self, key: &str) -> Option<&T>
    where
        Sliders: SliderMapForType<T>,
    {
        let map = self.map_for_type();
        return map.get(key);
    }
}

mod internal_traits {
    use std::collections::HashMap;

    pub trait SliderMapForType<T> {
        fn map_for_type(&self) -> &HashMap<String, T>;
        fn map_for_type_mut(&mut self) -> &mut HashMap<String, T>;
    }
}

impl SliderMapForType<Rgb> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, Rgb> {
        &self.colors
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, Rgb> {
        &mut self.colors
    }
}

impl SliderMapForType<f32> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, f32> {
        &self.f32s
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, f32> {
        &mut self.f32s
    }
}

impl SliderMapForType<bool> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, bool> {
        &self.bools
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, bool> {
        &mut self.bools
    }
}

impl SliderMapForType<Vec3> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, Vec3> {
        &self.vec3s
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, Vec3> {
        &mut self.vec3s
    }
}

impl SliderMapForType<usize> for Sliders {
    fn map_for_type(&self) -> &HashMap<String, usize> {
        &self.usizes
    }

    fn map_for_type_mut(&mut self) -> &mut HashMap<String, usize> {
        &mut self.usizes
    }
}

pub trait Slider<T>: SliderMapForType<T> {}
impl Slider<Rgb> for Sliders {}
impl Slider<f32> for Sliders {}
impl Slider<bool> for Sliders {}
impl Slider<Vec3> for Sliders {}
impl Slider<usize> for Sliders {}
