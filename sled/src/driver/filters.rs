use compact_str::{CompactString, ToCompactString};
use micromap::Map;

use crate::Filter;

pub struct Filters {
    map: Map<CompactString, Filter, 12>,
}

impl Default for Filters {
    fn default() -> Self {
        Self::new()
    }
}

impl Filters {
    pub fn new() -> Self {
        Filters { map: Map::new() }
    }

    pub fn set(&mut self, key: &str, value: Filter) {
        self.map.insert(key.to_compact_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Filter> {
        self.map.get(key)
    }
}
