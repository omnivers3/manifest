use url::{ Url };

/// General metadata about a package.
///
/// These fields are not validated, but rather accept any valid TOML specification values.
#[derive(PartialEq, Clone, Debug)]
pub struct Metadata {
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub description: Option<String>,
    /// Path to readme not content
    pub readme: Option<String>,
    pub homepage: Option<Url>,
    pub repository: Option<Url>,
    pub documentation: Option<Url>,
    pub badges: BTreeMap<String, BTreeMap<String, String>>,
    pub links: Option<String>,
}