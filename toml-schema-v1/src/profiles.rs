use crate::Profile;

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct Profiles {
    pub test: Option<Profile>,
    pub doc: Option<Profile>,
    pub bench: Option<Profile>,
    pub dev: Option<Profile>,
    pub release: Option<Profile>,
}