use std::any::type_name;

use compact_str::{CompactString, ToCompactString};
use glam::Vec2;
use palette::rgb::Rgb;
use std::collections::HashMap;

pub struct BufferContainer {
    f32s: HashMap<CompactString, Vec<f32>>,
    rgbs: HashMap<CompactString, Vec<Rgb>>,
    bools: HashMap<CompactString, Vec<bool>>,
    vec2s: HashMap<CompactString, Vec<Vec2>>,
    usizes: HashMap<CompactString, Vec<usize>>,
}

mod internal_traits {
    use super::BufferContainer;
    use super::{CompactString, HashMap};
    use super::{Rgb, Vec2};
    pub trait MapForType<T> {
        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<T>>;
        fn map_for_type(&self) -> &HashMap<CompactString, Vec<T>>;
    }

    impl MapForType<usize> for BufferContainer {
        fn map_for_type(&self) -> &HashMap<CompactString, Vec<usize>> {
            &self.usizes
        }

        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<usize>> {
            &mut self.usizes
        }
    }

    impl MapForType<bool> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<bool>> {
            &mut self.bools
        }

        fn map_for_type(&self) -> &HashMap<CompactString, Vec<bool>> {
            &self.bools
        }
    }

    impl MapForType<f32> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<f32>> {
            &mut self.f32s
        }

        fn map_for_type(&self) -> &HashMap<CompactString, Vec<f32>> {
            &self.f32s
        }
    }

    impl MapForType<Rgb> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<Rgb>> {
            &mut self.rgbs
        }

        fn map_for_type(&self) -> &HashMap<CompactString, Vec<Rgb>> {
            &self.rgbs
        }
    }

    impl MapForType<Vec2> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut HashMap<CompactString, Vec<Vec2>> {
            &mut self.vec2s
        }

        fn map_for_type(&self) -> &HashMap<CompactString, Vec<Vec2>> {
            &self.vec2s
        }
    }
}

use crate::SledError;

pub use self::internal_traits::MapForType;

impl BufferContainer {
    pub fn new() -> Self {
        BufferContainer {
            f32s: HashMap::new(),
            rgbs: HashMap::new(),
            bools: HashMap::new(),
            vec2s: HashMap::new(),
            usizes: HashMap::new(),
        }
    }

    pub fn create<T>(&mut self, buffer_name: &str) -> &mut Vec<T>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.insert(buffer_name.to_compact_string(), Vec::new());
        map.get_mut(buffer_name).unwrap()
    }

    pub fn get_buffer<T>(&self, buffer_name: &str) -> Option<&Vec<T>>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type();
        map.get(buffer_name)
    }

    pub fn get_buffer_mut<T>(&mut self, buffer_name: &str) -> Option<&mut Vec<T>>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.get_mut(buffer_name)
    }

    pub fn get<T>(&self, buffer_name: &str, index: usize) -> Option<T>
    where
        BufferContainer: MapForType<T>,
        T: Copy,
    {
        let buffer = self.get_buffer(buffer_name)?;
        deref_option(buffer.get(index))
    }

    pub fn set<T>(&mut self, buffer_name: &str, index: usize, value: T) -> Result<(), SledError>
    where
        BufferContainer: MapForType<T>,
    {
        let buffer = self.get_buffer_mut(buffer_name).ok_or_else(|| {
            SledError::new(format!(
                "There is no Vec<{}> with the name `{}`.",
                type_name::<T>(),
                buffer_name
            ))
        })?;

        buffer[index] = value;
        Ok(())
    }

    pub fn push<T>(&mut self, buffer_name: &str, value: T) -> Result<(), SledError>
    where
        BufferContainer: MapForType<T>,
    {
        let buffer = self.get_buffer_mut(buffer_name).ok_or_else(|| {
            SledError::new(format!(
                "There is no Vec<{}> with the name `{}`.",
                type_name::<T>(),
                buffer_name
            ))
        })?;

        buffer.push(value);
        Ok(())
    }
}

fn deref_option<T: Copy>(option: Option<&T>) -> Option<T> {
    option.map(|v| *v)
}

impl std::ops::Index<&str> for BufferContainer {
    type Output = Vec<f32>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.f32s[index]
    }
}
