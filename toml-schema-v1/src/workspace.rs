
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}