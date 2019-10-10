use crate::Profile;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Profiles {
    pub test: Option<Profile>,
    pub doc: Option<Profile>,
    pub bench: Option<Profile>,
    pub dev: Option<Profile>,
    pub release: Option<Profile>,
}

// parsing logic for profiles
//
// match name {
//     "dev" | "release" => {}
//     _ => {
//         if self.overrides.is_some() || self.build_override.is_some() {
//             bail!(
//                 "Profile overrides may only be specified for \
//                     `dev` or `release` profile, not `{}`.",
//                 name
//             );
//         }
//     }
// }

// match name {
//     "doc" => {
//         warnings.push("profile `doc` is deprecated and has no effect".to_string());
//     }
//     "test" | "bench" => {
//         if self.panic.is_some() {
//             warnings.push(format!("`panic` setting is ignored for `{}` profile", name))
//         }
//     }
//     _ => {}
// }