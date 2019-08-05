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


// impl TomlProfile {
//     pub fn validate(
//         &self,
//         name: &str,
//         features: &Features,
//         warnings: &mut Vec<String>,
//     ) -> CargoResult<()> {
//         if let Some(ref profile) = self.build_override {
//             features.require(Feature::profile_overrides())?;
//             profile.validate_override()?;
//         }
//         if let Some(ref override_map) = self.overrides {
//             features.require(Feature::profile_overrides())?;
//             for profile in override_map.values() {
//                 profile.validate_override()?;
//             }
//         }

//         match name {
//             "dev" | "release" => {}
//             _ => {
//                 if self.overrides.is_some() || self.build_override.is_some() {
//                     bail!(
//                         "Profile overrides may only be specified for \
//                          `dev` or `release` profile, not `{}`.",
//                         name
//                     );
//                 }
//             }
//         }

//         match name {
//             "doc" => {
//                 warnings.push("profile `doc` is deprecated and has no effect".to_string());
//             }
//             "test" | "bench" => {
//                 if self.panic.is_some() {
//                     warnings.push(format!("`panic` setting is ignored for `{}` profile", name))
//                 }
//             }
//             _ => {}
//         }

//         if let Some(panic) = &self.panic {
//             if panic != "unwind" && panic != "abort" {
//                 bail!(
//                     "`panic` setting of `{}` is not a valid setting,\
//                      must be `unwind` or `abort`",
//                     panic
//                 );
//             }
//         }
//         Ok(())
//     }

//     fn validate_override(&self) -> CargoResult<()> {
//         if self.overrides.is_some() || self.build_override.is_some() {
//             bail!("Profile overrides cannot be nested.");
//         }
//         if self.panic.is_some() {
//             bail!("`panic` may not be specified in a profile override.")
//         }
//         if self.lto.is_some() {
//             bail!("`lto` may not be specified in a profile override.")
//         }
//         if self.rpath.is_some() {
//             bail!("`rpath` may not be specified in a profile override.")
//         }
//         Ok(())
//     }
// }
