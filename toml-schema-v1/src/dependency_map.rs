use std::collections::{ BTreeMap };

use crate::{ Dependency };

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyMap(pub BTreeMap<String, Dependency>);

impl std::ops::Deref for DependencyMap {
    type Target = BTreeMap<String, Dependency>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::cmp::PartialEq for DependencyMap {
    fn eq(&self, other: &Self) -> bool {
        let mut other_keys: Vec<&String> = other.keys().collect();
        let keys = self.keys();
        let len = keys.len();
        if other_keys.len() != len { // Differnt sized key spaces
            return false;
        }
        if len == 0 { // No keys on either side
            return true;
        }
        for i in 0..other_keys.len() {
            // If any key isn't found then equality fails
            if !self.contains_key(other_keys[i]) { return false; }
            other_keys.remove(i);
        }
        // If there are any remaining keys that weren't matched then equality fails
        if other_keys.len() != 0 {
            return false;
        }
        true
    }
}