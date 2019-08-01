use crate::{ VecStringOrBool };
use crate::string_or_vec::StringOrVec;
use crate::string_or_bool::StringOrBool;

/// Represents the `package`/`project` sections of a `Cargo.toml`.
///
/// Note that the order of the fields matters, since this is the order they
/// are serialized to a TOML file. For example, you cannot have values after
/// the field `metadata`, since it is a table and values cannot appear after
/// tables.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Project {
    pub edition: Option<String>,
    pub name: String,
    pub version: semver::Version,
    pub authors: Option<Vec<String>>,
    pub build: Option<StringOrBool>,
    pub metabuild: Option<StringOrVec>,
    pub links: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub publish: Option<VecStringOrBool>,
    pub publish_lockfile: Option<bool>,
    pub workspace: Option<String>,
    // im_a_teapot: Option<bool>,
    pub autobins: Option<bool>,
    pub autoexamples: Option<bool>,
    pub autotests: Option<bool>,
    pub autobenches: Option<bool>,
    pub namespaced_features: Option<bool>,
    pub default_run: Option<String>,

    // Package metadata.
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub metadata: Option<toml::Value>,
}