use std::collections::{ BTreeMap };

use crate::{ DependencyMap, Platform, Profiles, Project, Target, Workspace };

pub type LibTarget = Target;
pub type BinTarget = Target;
pub type ExampleTarget = Target;
pub type TestTarget = Target;
pub type BenchTarget = Target;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    pub cargo_features: Option<Vec<String>>,
    pub package: Option<Project>,
    pub project: Option<Project>,
    pub profile: Option<Profiles>,
    pub lib: Option<LibTarget>,
    pub bin: Option<Vec<BinTarget>>,
    pub example: Option<Vec<ExampleTarget>>,
    pub test: Option<Vec<TestTarget>>,
    pub bench: Option<Vec<TestTarget>>,
    pub dependencies: Option<DependencyMap>,
    pub dev_dependencies: Option<DependencyMap>,
    pub build_dependencies: Option<DependencyMap>,
    pub features: Option<BTreeMap<String, Vec<String>>>,
    pub target: Option<BTreeMap<String, Platform>>,
    pub replace: Option<DependencyMap>,
    pub patch: Option<BTreeMap<String, DependencyMap>>,
    pub workspace: Option<Workspace>,
    pub badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
}