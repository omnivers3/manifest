use crate::Profile;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Profiles {
    pub test: Option<Profile>,
    pub doc: Option<Profile>,
    pub bench: Option<Profile>,
    pub dev: Option<Profile>,
    pub release: Option<Profile>,
}

// util/toml/mod.rs - line 280
// impl Profiles {
//     pub fn validate(&self, features: &Features, warnings: &mut Vec<String>) -> Result<()> {
//         if let Some(ref test) = self.test {
//             test.validate("test", features, warnings)?;
//         }
//         if let Some(ref doc) = self.doc {
//             doc.validate("doc", features, warnings)?;
//         }
//         if let Some(ref bench) = self.bench {
//             bench.validate("bench", features, warnings)?;
//         }
//         if let Some(ref dev) = self.dev {
//             dev.validate("dev", features, warnings)?;
//         }
//         if let Some(ref release) = self.release {
//             release.validate("release", features, warnings)?;
//         }
//         Ok(())
//     }
// }