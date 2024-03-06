use std::any::{type_name, Any};

use compact_str::{CompactString, ToCompactString};
use std::collections::HashMap;

use crate::SledError;

pub struct BufferContainer {
    buffers: HashMap<CompactString, Box<dyn Buffer>>,
}

impl Default for BufferContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferContainer {
    pub fn new() -> Self {
        BufferContainer {
            buffers: HashMap::new(),
        }
    }

    pub fn create_buffer<T: BufferableData>(&mut self, key: &str) -> &mut Vec<T> {
        self.buffers
            .insert(key.to_compact_string(), Box::<Vec<T>>::default());
        self.get_buffer_mut(key).unwrap()
    }

    pub fn get_buffer<T: BufferableData>(&self, key: &str) -> Result<&Vec<T>, SledError> {
        let buffer_trait_obj = self.buffers.get(key).ok_or_else(|| {
            SledError::new(format!("There is no Buffer with the name `{}`.", key))
        })?;

        buffer_trait_obj
            .as_any()
            .downcast_ref::<Vec<T>>()
            .ok_or_else(|| {
                SledError::new(format!(
                    "Buffer with name `{}` exists but it is not a buffer of {} values.",
                    key,
                    type_name::<T>()
                ))
            })
    }

    pub fn get_buffer_mut<T: BufferableData>(
        &mut self,
        key: &str,
    ) -> Result<&mut Vec<T>, SledError> {
        let buffer_trait_obj = self.buffers.get_mut(key).ok_or_else(|| {
            SledError::new(format!("There is no Buffer with the name `{}`.", key))
        })?;

        buffer_trait_obj
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .ok_or_else(|| {
                SledError::new(format!(
                    "Buffer with name `{}` exists but it is not a buffer of {} values.",
                    key,
                    type_name::<T>()
                ))
            })
    }

    pub fn get_buffer_item<T: BufferableData>(
        &self,
        key: &str,
        index: usize,
    ) -> Result<&T, SledError> {
        let buffer = self.get_buffer(key)?;
        buffer
            .get(index)
            .ok_or_else(|| SledError::new(format!("Buffer has no item at index {}", index)))
    }

    pub fn get_buffer_item_mut<T: BufferableData>(
        &mut self,
        key: &str,
        index: usize,
    ) -> Result<&mut T, SledError> {
        let buffer = self.get_buffer_mut(key)?;
        buffer
            .get_mut(index)
            .ok_or_else(|| SledError::new(format!("Buffer has no item at index {}", index)))
    }

    pub fn set_buffer_item<T: BufferableData>(
        &mut self,
        key: &str,
        index: usize,
        value: T,
    ) -> Result<(), SledError> {
        *self.get_buffer_item_mut(key, index)? = value;
        Ok(())
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<CompactString, Box<dyn Buffer>> {
        self.buffers.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<CompactString, Box<dyn Buffer>> {
        self.buffers.iter_mut()
    }
}

impl std::iter::IntoIterator for BufferContainer {
    type Item = (CompactString, Box<dyn Buffer>);
    type IntoIter = std::collections::hash_map::IntoIter<CompactString, Box<dyn Buffer>>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffers.into_iter()
    }
}

pub trait Buffer {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait BufferableData: 'static {}
impl<T: Sized + 'static> BufferableData for T {}

impl<T: BufferableData> Buffer for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
