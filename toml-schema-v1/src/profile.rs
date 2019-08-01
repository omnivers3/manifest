use std::collections::{ BTreeMap };

use crate::{ OptLevel, ProfilePackageSpec, U32OrBool };
use crate::string_or_bool::{ StringOrBool };

#[derive(Deserialize, Serialize, Clone, Debug, Default, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub opt_level: Option<OptLevel>,
    pub lto: Option<StringOrBool>,
    pub codegen_units: Option<u32>,
    pub debug: Option<U32OrBool>,
    pub debug_assertions: Option<bool>,
    pub rpath: Option<bool>,
    pub panic: Option<String>,
    pub overflow_checks: Option<bool>,
    pub incremental: Option<bool>,
    pub overrides: Option<BTreeMap<ProfilePackageSpec, Profile>>,
    pub build_override: Option<Box<Profile>>,
}