use alloc::collections::BTreeMap;
use alloc::format;

use compact_str::{CompactString, ToCompactString};

use crate::{Filter, SledError};

#[derive(Clone, Debug)]
pub struct Filters {
    map: BTreeMap<CompactString, Filter>,
}

impl Default for Filters {
    fn default() -> Self {
        Self::new()
    }
}

impl Filters {
    pub fn new() -> Self {
        Filters {
            map: BTreeMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Filter) {
        self.map.insert(key.to_compact_string(), value);
    }

    pub fn get(&self, key: &str) -> Result<&Filter, SledError> {
        self.map
            .get(key)
            .ok_or_else(|| SledError::new(format!("No filter found with key '{}'", key)))
    }

    pub fn iter(&self) -> alloc::collections::btree_map::Iter<CompactString, Filter> {
        self.map.iter()
    }
}

impl IntoIterator for Filters {
    type Item = (CompactString, Filter);
    type IntoIter = alloc::collections::btree_map::IntoIter<CompactString, Filter>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl FromIterator<(CompactString, Filter)> for Filters {
    fn from_iter<T: IntoIterator<Item = (CompactString, Filter)>>(iter: T) -> Self {
        let mut f = Filters::new();

        for (key, value) in iter {
            f.map.insert(key, value);
        }

        f
    }
}

impl Extend<(CompactString, Filter)> for Filters {
    fn extend<T: IntoIterator<Item = (CompactString, Filter)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.map.insert(key, value);
        }
    }
}
