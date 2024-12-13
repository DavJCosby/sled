use core::{any::Any, fmt::Debug};

use alloc::collections::BTreeMap;
use compact_str::{CompactString, ToCompactString};

use crate::SledError;

#[derive(Debug)]
pub struct Data {
    data: BTreeMap<CompactString, Box<dyn Downcastable>>,
}

#[derive(Debug)]
struct DataWrapper<T>(T);

impl<T> DataWrapper<T> {
    pub fn new(value: T) -> Self {
        DataWrapper(value)
    }
}

trait Downcastable: Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl<T: StorableData + Debug> Downcastable for DataWrapper<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait StorableData: 'static + Debug {}
impl<T: Sized + 'static + Debug> StorableData for T {}

impl Data {
    pub fn new() -> Self {
        Data {
            data: BTreeMap::new(),
        }
    }

    pub fn get<T: StorableData>(&self, key: &str) -> Result<&T, SledError> {
        let candidate = self
            .data
            .get(key)
            .ok_or_else(|| SledError::new(format!("No data associated with the key `{}`.", key)))?;

        match candidate.as_any().downcast_ref::<DataWrapper<T>>() {
            Some(wrapper) => Ok(&wrapper.0),
            None => Err(SledError::new(format!(
                "Data with the key `{}` exists but it is not of type {}.",
                key,
                core::any::type_name::<T>()
            ))),
        }
    }

    pub fn get_mut<T: StorableData>(&mut self, key: &str) -> Result<&mut T, SledError> {
        let candidate = self
            .data
            .get_mut(key)
            .ok_or_else(|| SledError::new(format!("No data associated with the key `{}`.", key)))?;

        match candidate.as_any_mut().downcast_mut::<DataWrapper<T>>() {
            Some(wrapper) => Ok(&mut wrapper.0),
            None => Err(SledError::new(format!(
                "Data with the key `{}` exists but it is not of type {}.",
                key,
                core::any::type_name::<T>()
            ))),
        }
    }

    pub fn set<T: StorableData>(&mut self, key: &str, value: T) {
        self.data.insert(
            key.to_compact_string(),
            Box::<DataWrapper<T>>::new(DataWrapper::new(value)),
        );
    }

    pub fn store<T: StorableData>(&mut self, key: &str, value: T) -> &mut T {
        self.data.insert(
            key.to_compact_string(),
            Box::<DataWrapper<T>>::new(DataWrapper::new(value)),
        );
        self.get_mut(key).unwrap()
    }

    pub fn empty_at(&self, key: &str) -> bool {
        !self.data.contains_key(key)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}
