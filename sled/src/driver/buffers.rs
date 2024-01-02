use std::any::type_name;

use compact_str::{CompactString, ToCompactString};
use glam::Vec2;
use micromap::Map;
use palette::rgb::Rgb;
use smallvec::SmallVec;

const MAX_BUFFERS: usize = 4;
const DEFAULT_BUFFER_SIZE: usize = 12;

pub trait BufferDataStructure {}
impl<A: smallvec::Array> BufferDataStructure for SmallVec<A> {}

pub type Buffer<T, const N: usize = DEFAULT_BUFFER_SIZE> = SmallVec<[T; N]>;

pub struct BufferContainer {
    f32s: Map<CompactString, Buffer<f32>, MAX_BUFFERS>,
    rgbs: Map<CompactString, Buffer<Rgb>, MAX_BUFFERS>,
    bools: Map<CompactString, Buffer<bool>, MAX_BUFFERS>,
    vec2s: Map<CompactString, Buffer<Vec2>, MAX_BUFFERS>,
    usizes: Map<CompactString, Buffer<usize>, MAX_BUFFERS>,
}

mod internal_traits {
    use super::{Buffer, BufferContainer};
    use super::{CompactString, Map, MAX_BUFFERS};
    use super::{Rgb, Vec2};
    pub trait MapForType<T> {
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<T>, MAX_BUFFERS>;
        fn map_for_type(&self) -> &Map<CompactString, Buffer<T>, MAX_BUFFERS>;
    }

    impl MapForType<usize> for BufferContainer {
        fn map_for_type(&self) -> &Map<CompactString, Buffer<usize>, MAX_BUFFERS> {
            &self.usizes
        }

        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<usize>, MAX_BUFFERS> {
            &mut self.usizes
        }
    }

    impl MapForType<bool> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<bool>, MAX_BUFFERS> {
            &mut self.bools
        }

        fn map_for_type(&self) -> &Map<CompactString, Buffer<bool>, MAX_BUFFERS> {
            &self.bools
        }
    }

    impl MapForType<f32> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<f32>, MAX_BUFFERS> {
            &mut self.f32s
        }

        fn map_for_type(&self) -> &Map<CompactString, Buffer<f32>, MAX_BUFFERS> {
            &self.f32s
        }
    }

    impl MapForType<Rgb> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<Rgb>, MAX_BUFFERS> {
            &mut self.rgbs
        }

        fn map_for_type(&self) -> &Map<CompactString, Buffer<Rgb>, MAX_BUFFERS> {
            &self.rgbs
        }
    }

    impl MapForType<Vec2> for BufferContainer {
        fn map_for_type_mut(&mut self) -> &mut Map<CompactString, Buffer<Vec2>, MAX_BUFFERS> {
            &mut self.vec2s
        }

        fn map_for_type(&self) -> &Map<CompactString, Buffer<Vec2>, MAX_BUFFERS> {
            &self.vec2s
        }
    }
}

use crate::SledError;

pub use self::internal_traits::MapForType;

#[allow(dead_code)]
impl BufferContainer {
    pub fn new() -> Self {
        BufferContainer {
            f32s: Map::new(),
            rgbs: Map::new(),
            bools: Map::new(),
            vec2s: Map::new(),
            usizes: Map::new(),
        }
    }

    pub fn create<T>(&mut self, buffer_name: &str) -> &mut Buffer<T>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.insert(buffer_name.to_compact_string(), Buffer::new());
        &mut map[buffer_name]
    }

    pub fn get_buffer<T>(&self, buffer_name: &str) -> Option<&Buffer<T>>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type();
        map.get(buffer_name)
    }

    pub fn get_buffer_mut<T>(&mut self, buffer_name: &str) -> Option<&mut Buffer<T>>
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
                "There is no Buffer<{}> with the name `{}`.",
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
                "There is no Buffer<{}> with the name `{}`.",
                type_name::<T>(),
                buffer_name
            ))
        })?;

        buffer.push(value);
        Ok(())
    }
}

fn deref_option<T: Copy>(option: Option<&T>) -> Option<T> {
    match option {
        Some(v) => Some(*v),
        None => None,
    }
}

trait TypedIndex<T> {}

impl std::ops::Index<&str> for BufferContainer {
    type Output = Buffer<f32>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.f32s[index]
    }
}
