use crate::{ PathValue };

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Target {
    pub name: Option<String>,
    pub crate_type: Option<Vec<String>>,
    pub path: Option<PathValue>,
    pub test: Option<bool>,
    pub doctest: Option<bool>,
    pub bench: Option<bool>,
    pub doc: Option<bool>,
    pub plugin: Option<bool>,
    pub proc_macro: Option<bool>,
    pub harness: Option<bool>,
    pub required_features: Option<Vec<String>>,
    pub edition: Option<String>,
}

// /// Checks a list of build targets, and ensures the target names are unique within a vector.
// /// If not, the name of the offending build target is returned.
// fn unique_build_targets(targets: &[Target], package_root: &Path) -> Result<(), String> {
//     let mut seen = HashSet::new();
//     for target in targets {
//         if let TargetSourcePath::Path(path) = target.src_path() {
//             let full = package_root.join(path);
//             if !seen.insert(full.clone()) {
//                 return Err(full.display().to_string());
//             }
//         }
//     }
//     Ok(())
// }

// impl TomlTarget {
//     fn new() -> TomlTarget {
//         TomlTarget::default()
//     }

//     fn name(&self) -> String {
//         match self.name {
//             Some(ref name) => name.clone(),
//             None => panic!("target name is required"),
//         }
//     }

//     fn proc_macro(&self) -> Option<bool> {
//         self.proc_macro.or(self.proc_macro2).or_else(|| {
//             if let Some(types) = self.crate_types() {
//                 if types.contains(&"proc-macro".to_string()) {
//                     return Some(true);
//                 }
//             }
//             None
//         })
//     }

//     fn crate_types(&self) -> Option<&Vec<String>> {
//         self.crate_type
//             .as_ref()
//             .or_else(|| self.crate_type2.as_ref())
//     }
// }