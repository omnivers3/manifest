#[derive(Clone, Debug, PartialEq)]
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}