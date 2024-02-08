use crate::Filter;
use compact_str::{CompactString, ToCompactString};
use std::collections::HashMap;

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
}
