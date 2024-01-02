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

    pub fn create_buffer<T>(&mut self, key: &str) -> &mut Buffer<T>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.insert(key.to_compact_string(), Buffer::new());
        &mut map[key]
    }

    pub fn get<T>(&self, key: &str) -> Option<&Buffer<T>>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type();
        map.get(key)
    }

    pub fn get_mut<T>(&mut self, key: &str) -> Option<&mut Buffer<T>>
    where
        BufferContainer: MapForType<T>,
    {
        let map = self.map_for_type_mut();
        map.get_mut(key)
    }
}

trait TypedIndex<T> {}

impl std::ops::Index<&str> for BufferContainer {
    type Output = Buffer<f32>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.f32s[index]
    }
}

#[allow(dead_code)]
fn main() {
    let mut t = BufferContainer::new();
    let _buf = t.create_buffer::<f32>("numbers");
    let b2 = t.create_buffer("strs");

    b2.push(12.0);
}
