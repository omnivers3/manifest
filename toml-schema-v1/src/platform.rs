use crate::{ DependencyMap };

/// Corresponds to a `target` entry, but `Target` is already used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Platform {
    pub dependencies: Option<DependencyMap>,
    pub build_dependencies: Option<DependencyMap>,
    pub dev_dependencies: Option<DependencyMap>,
}