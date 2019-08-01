use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub registry: Option<String>,
    /// The URL of the `registry` field.
    /// This is an internal implementation detail. When Cargo creates a
    /// package, it replaces `registry` with `registry-index` so that the
    /// manifest contains the correct URL. All users won't have the same
    /// registry names configured, so Cargo can't rely on just the name for
    /// crates published by other users.
    pub registry_index: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub features: Option<Vec<String>>,
    pub optional: Option<bool>,
    pub default_features: Option<bool>,
    pub package: Option<String>,
    pub public: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}