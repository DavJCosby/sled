use std::{
    any::{type_name, Any},
    fmt::Debug,
};

use compact_str::{CompactString, ToCompactString};
use std::collections::HashMap;

use crate::SledError;

#[derive(Debug)]
pub struct BufferContainer {
    buffers: HashMap<CompactString, Box<dyn Buffer>>,
}

impl BufferContainer {
    pub fn new() -> Self {
        BufferContainer {
            buffers: HashMap::new(),
        }
    }

    /// Creates a `Vec<T>` and associates it to the given key in the BufferContainer, and then returns a mutable reference to it.
    ///
    /// Keys may be no longer than 24 bytes (or 12 bytes of 32-bit architectures) as we internally store them as [CompactString](https://docs.rs/compact_str/latest/compact_str/)s under the hood to minimize overhead.
    ///
    /// ```rust
    /// # use sled::{Sled, color::Rgb, SledError};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let color_buffer = buffers.create_buffer::<Rgb>("colors");
    /// color_buffer.extend([
    ///     Rgb::new(1.0, 0.0, 0.0),
    ///     Rgb::new(0.0, 0.0, 1.0),
    ///     Rgb::new(0.0, 1.0, 0.0),
    /// ]);
    ///
    /// assert_eq!(buffers.get_buffer::<Rgb>("colors")?.len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_buffer<T: BufferableData + Debug>(&mut self, key: &str) -> &mut Vec<T> {
        #[cfg(target_pointer_width = "64")]
        assert!(
            key.len() < 24,
            "Invalid buffer key; Max size is 24 bytes, `{}` is {} bytes.",
            key,
            key.len()
        );

        #[cfg(target_pointer_width = "32")]
        assert!(
            key.len() < 24,
            "Invalid buffer key; Max size is 12 bytes on 32-bit systems, `{}` is {} bytes.",
            key,
            key.len()
        );

        self.buffers
            .insert(key.to_compact_string(), Box::<Vec<T>>::default());
        self.get_buffer_mut(key).unwrap()
    }

    /// Returns `Ok(&Vec<T>)` if there is a buffer of type `T` associated with the given key.
    ///
    /// Otherwise, returns a [SledError].
    ///
    /// ```rust
    /// # use sled::{Sled, SledError};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let num_buffer = buffers.create_buffer::<usize>("numbers");
    /// num_buffer.extend([1, 5, 7]);
    ///
    /// let numbers: &Vec<usize> = buffers.get_buffer("numbers")?;
    /// let sum: usize = numbers.iter().sum();
    /// assert_eq!(sum, 13);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns `Ok(&mut Vec<T>)` if there is a buffer of type `T` associated with the given key.
    ///
    /// Otherwise, returns a [SledError].
    ///
    /// ```rust
    /// # use sled::{Sled, SledError};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let flag_buffer = buffers.create_buffer::<bool>("flags");
    /// flag_buffer.extend([true, false, false, true, false]);
    ///
    /// let flags = buffers.get_buffer_mut::<bool>("flags")?;
    /// for flag in flags {
    ///     *flag = !(*flag);
    /// }
    ///
    /// assert_eq!(buffers.get_buffer_item::<bool>("flags", 1)?, &true);
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns `Ok(&T)` if there is a buffer of type `T` associated with the given key and it has an item at the provided index.
    ///
    /// Otherwise, returns a [SledError].
    ///
    /// ```rust
    /// # use sled::{Sled, SledError, Vec2};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let pos_buffer = buffers.create_buffer::<Vec2>("positions");
    /// pos_buffer.extend([
    ///     Vec2::new(0.1, 0.0),
    ///     Vec2::new(0.2, 0.0),
    ///     Vec2::new(0.3, 0.0)
    /// ]);
    ///
    /// let first_item = buffers.get_buffer_item::<Vec2>("positions", 0)?;
    /// assert_eq!(first_item, &Vec2::new(0.1, 0.0));
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns `Ok(&mut T)` if there is a buffer of type `T` associated with the given key and it has an item at the provided index.
    ///
    /// Otherwise, returns a [SledError].
    ///
    /// ```rust
    /// # use sled::{Sled, SledError, Vec2};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let pos_buffer = buffers.create_buffer::<Vec2>("positions");
    /// pos_buffer.extend([
    ///     Vec2::new(0.1, 0.0),
    ///     Vec2::new(0.2, 0.0),
    ///     Vec2::new(0.3, 0.0)
    /// ]);
    ///
    /// let first_item: &mut Vec2 = buffers.get_buffer_item_mut("positions", 0)?;
    /// *first_item += Vec2::new(0.0, 1.0);
    ///
    /// assert_eq!(
    ///     buffers.get_buffer_item::<Vec2>("positions", 0)?,
    ///     &Vec2::new(0.1, 1.0)
    /// );
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns `Ok()` if we are able to successfully find a buffer of type `T` associated with the given key and set the value at the given index to `value.`
    ///
    /// Otherwise, returns a [SledError].
    ///
    /// ```rust
    /// # use sled::{Sled, SledError, Vec2};
    /// # use sled::BufferContainer;
    /// # fn main() -> Result<(), sled::SledError> {
    /// let mut buffers = BufferContainer::new();
    /// let float_buffer = buffers.create_buffer::<f32>("floats");
    /// float_buffer.extend([1.1, 2.2, 3.5, 4.4]);
    ///
    /// buffers.set_buffer_item::<f32>("floats", 2, 3.3)?;
    ///
    /// assert_eq!(buffers.get_buffer_item::<f32>("floats", 2)?, &3.3);
    /// # Ok(())
    /// # }
    /// ```
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

pub trait Buffer: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait BufferableData: 'static {}
impl<T: Sized + 'static> BufferableData for T {}

impl<T: BufferableData + Debug> Buffer for Vec<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl std::iter::IntoIterator for BufferContainer {
    type Item = (CompactString, Box<dyn Buffer>);
    type IntoIter = std::collections::hash_map::IntoIter<CompactString, Box<dyn Buffer>>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffers.into_iter()
    }
}

impl std::iter::FromIterator<(CompactString, Box<dyn Buffer>)> for BufferContainer {
    fn from_iter<T: IntoIterator<Item = (CompactString, Box<dyn Buffer>)>>(iter: T) -> Self {
        let mut bc = BufferContainer::new();

        for (key, value) in iter {
            bc.buffers.insert(key, value);
        }

        bc
    }
}

impl std::iter::Extend<(CompactString, Box<dyn Buffer>)> for BufferContainer {
    fn extend<T: IntoIterator<Item = (CompactString, Box<dyn Buffer>)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.buffers.insert(key, value);
        }
    }
}

impl Default for BufferContainer {
    fn default() -> Self {
        Self::new()
    }
}
