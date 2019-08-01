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