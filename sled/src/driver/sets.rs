use compact_str::{CompactString, ToCompactString};
use micromap::Map;

use crate::Set;

pub struct Sets {
    map: Map<CompactString, Set, 12>,
}

impl Default for Sets {
    fn default() -> Self {
        Self::new()
    }
}

impl Sets {
    pub fn new() -> Self {
        Sets { map: Map::new() }
    }

    pub fn set(&mut self, key: &str, value: Set) {
        self.map.insert(key.to_compact_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Set> {
        self.map.get(key)
    }
}
