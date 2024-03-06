use crate::Filter;
use compact_str::{CompactString, ToCompactString};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Filters {
    map: HashMap<CompactString, Filter>,
}

impl Default for Filters {
    fn default() -> Self {
        Self::new()
    }
}

impl Filters {
    pub fn new() -> Self {
        Filters {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Filter) {
        self.map.insert(key.to_compact_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Filter> {
        self.map.get(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<CompactString, Filter> {
        self.map.iter()
    }
}

impl IntoIterator for Filters {
    type Item = (CompactString, Filter);
    type IntoIter = std::collections::hash_map::IntoIter<CompactString, Filter>;

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
