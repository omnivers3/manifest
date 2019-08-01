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
        self.keys().eq(other.keys()) && self.values().eq(other.values())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{ BTreeMap };

    use crate::{ Dependency, DependencyMap };

    #[test]
    fn empty_maps_should_be_equal() {
        let dm1 = DependencyMap(BTreeMap::default());
        let dm2 = DependencyMap(BTreeMap::default());

        assert_eq!(dm1, dm2);
    }

    #[test]
    fn empty_maps_should_not_be_equal_to_non_empty() {
        let dm1 = DependencyMap(BTreeMap::default());

        let mut dm2 = BTreeMap::new();
        dm2.insert("a".to_owned(), Dependency::Simple("".to_owned()));

        assert_ne!(dm1, DependencyMap(dm2));
    }

    #[test]
    fn maps_with_the_same_values_should_be_equal() {
        let mut dm1 = BTreeMap::default();
        dm1.insert("a".to_owned(), Dependency::Simple("".to_owned()));

        let mut dm2 = BTreeMap::new();
        dm2.insert("a".to_owned(), Dependency::Simple("".to_owned()));

        assert_eq!(DependencyMap(dm1), DependencyMap(dm2));
    }

    #[test]
    fn multi_value_maps_with_the_same_values_in_different_insert_order_should_be_equal() {
        let mut dm1 = BTreeMap::default();
        dm1.insert("a".to_owned(), Dependency::Simple("".to_owned()));
        dm1.insert("b".to_owned(), Dependency::Simple("2".to_owned()));

        let mut dm2 = BTreeMap::new();
        dm2.insert("b".to_owned(), Dependency::Simple("2".to_owned()));
        dm2.insert("a".to_owned(), Dependency::Simple("".to_owned()));

        assert_eq!(DependencyMap(dm1), DependencyMap(dm2));
    }

    #[test]
    fn multi_value_maps_with_different_values_and_different_insert_order_should_not_be_equal() {
        let mut dm1 = BTreeMap::default();
        dm1.insert("a".to_owned(), Dependency::Simple("".to_owned()));
        dm1.insert("b".to_owned(), Dependency::Simple("2".to_owned()));

        let mut dm2 = BTreeMap::new();
        dm2.insert("b".to_owned(), Dependency::Simple("3".to_owned()));
        dm2.insert("a".to_owned(), Dependency::Simple("".to_owned()));

        assert_ne!(DependencyMap(dm1), DependencyMap(dm2));
    }
}