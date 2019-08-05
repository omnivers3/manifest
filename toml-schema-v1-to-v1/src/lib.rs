extern crate failure;

extern crate omni_manifest_v1 as v1;
extern crate omni_manifest_toml_schema_v1 as schema_v1;

use failure::{ Fail };
use std::fmt;

pub const GIT_KEY_BRANCH: &'static str = "branch";
pub const GIT_KEY_TAG: &'static str = "tag";
pub const GIT_KEY_REV: &'static str = "rev";

#[derive(Debug, Fail, PartialEq)]
pub enum Warning {
    GitKeysIgnored(Vec<String>),
    IgnoredMetadata(String),
    NoValidSources(schema_v1::DetailedDependency),
    RegistryIgnored(String),
}

impl fmt::Display for Warning{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::GitKeysIgnored(keys) => write!(f, "git value for keys '{:?}' is ignored for this dependency", keys),
            Warning::IgnoredMetadata(version) => write!(f, "version requirement '{}' includes semver metadata which will be ignored and should be removed to avoid confusion", version),
            Warning::NoValidSources(target) => write!(f, "dependency ({:?}) specified without providing a local path, Git repository or version to use", target),
            Warning::RegistryIgnored(registry) => write!(f, "registry ({:?}) is not used", registry),
        }
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum Constraint {
    OneOfGitOrRegistry,
    OneOfGitOrPath,
    OneOfBranchTagOrRev,
}

impl fmt::Display for Constraint{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Constraint::OneOfGitOrRegistry => "Only one of 'git' or 'registry' is allowed.",
            Constraint::OneOfGitOrPath => "Only one of 'git' or 'path' is allowed.",
            Constraint::OneOfBranchTagOrRev => "Only one of 'branch', 'tag' or 'rev' allowed.",
        })
    }
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    Constraint(Constraint),
    V1(v1::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Constraint(c) => write!(f, "Constraint violated: {}", c),
            Error::V1(err) => err.fmt(f),
        }
    }
}

impl From<v1::Error> for Error {
    fn from(err: v1::Error) -> Error {
        Error::V1(err)
    }
}

pub type Result<T> = std::result::Result<(T, Option<Vec<Warning>>), Error>;

fn version_metadata_warning(src: &schema_v1::DetailedDependency) -> Option<Warning> {
    src.version.as_ref().and_then(|v|
        if v.contains('+') {
            Some(Warning::IgnoredMetadata(v.to_owned()))
        } else {
            None
        }
    )
}

fn git_missing_with_keys(src: &schema_v1::DetailedDependency) -> Option<Warning> {
    let has_git = src.git.is_some();
    let has_branch = src.branch.is_some();
    let has_tag = src.tag.is_some();
    let has_rev = src.rev.is_some();
    
    if has_git || ( !has_branch || !has_tag || !has_rev ) { return None; }
    
    let mut keys = vec![];
    if has_branch { keys.push(GIT_KEY_BRANCH.to_owned()) }
    if has_tag { keys.push(GIT_KEY_TAG.to_owned()) }
    if has_rev { keys.push(GIT_KEY_REV.to_owned()) }
    Some(Warning::GitKeysIgnored(keys))
}

fn convert_detailed_dependency(src: &schema_v1::DetailedDependency) -> Result<v1::Dependency> {
    let mut warnings = vec![];
    if src.version.as_ref()
        .or(src.path.as_ref())
        .or(src.git.as_ref())
        .is_none()
    {
        warnings.push(Warning::NoValidSources(src.to_owned()));
    }
    version_metadata_warning(src).map(|w|warnings.push(w));
    git_missing_with_keys(src).map(|w|warnings.push(w));
    
    let has_git = src.git.is_some();
    let has_registry = src.registry.is_some();
    let has_path = src.path.is_some();

    let dep = 
        if has_git {
            if has_registry { return Err(Error::Constraint(Constraint::OneOfGitOrRegistry)); }
            if has_path { return Err(Error::Constraint(Constraint::OneOfGitOrPath)); }

            let has_branch = src.branch.is_some();
            let has_tag = src.tag.is_some();
            let has_rev = src.rev.is_some();

            let git = src.git.to_owned().unwrap();
            let mut repo = v1::GitRepository::from_url_string(git)?;

            if has_branch {
                if has_tag || has_rev { return Err(Error::Constraint(Constraint::OneOfBranchTagOrRev)); }

                let branch = src.branch.as_ref().unwrap().to_owned();
                repo.reference = v1::GitReference::Branch(branch);
            } else if has_tag {
                if has_rev { return Err(Error::Constraint(Constraint::OneOfBranchTagOrRev)); }

                let tag = src.tag.as_ref().unwrap().to_owned();
                repo.reference = v1::GitReference::Tag(tag);
            } else if has_rev {
                let rev = src.rev.as_ref().unwrap().to_owned();
                repo.reference = v1::GitReference::Rev(rev);
            }
            v1::Dependency::Git(repo)
        } else if has_path { // path to local dir
            if let Some(registry) = src.registry.as_ref() {
                warnings.push(Warning::RegistryIgnored(registry.to_owned()))
            }
            let path = src.path.to_owned().unwrap();
            v1::Dependency::LocalPath(path.into())

        } else if has_registry { // custom registry
            v1::Dependency::CustomRegistry(src.registry.to_owned().unwrap())
        } else {
            let version = src.version.to_owned();
            v1::Dependency::DefaultRegistry(version.unwrap_or("".to_owned()))
        };

    let warnings = if warnings.len() == 0 { None } else { Some(warnings) };
    Ok((dep, warnings))
}

pub fn convert_dependency(src: &schema_v1::Dependency) -> Result<v1::Dependency> {
    match src {
        schema_v1::Dependency::Simple(value) => {
            // Ok((v1::Dependency::Named(name.to_owned()), None)),
            Ok((v1::Dependency::DefaultRegistry(value.to_owned()), None))
        },
        schema_v1::Dependency::Detailed(details) => convert_detailed_dependency(&details),
    }
}

#[cfg(test)]
mod tests {
    use v1;
    use schema_v1;

    use crate::{ convert_dependency, Error };

    #[test]
    fn convert_named_dependency_without_warnings() {
        let dep = schema_v1::Dependency::Simple("1.0.0".to_owned());
        match convert_dependency(&dep) {
            Ok ((d, w)) => {
                assert_eq!(None, w);
                assert_eq!(v1::Dependency::DefaultRegistry("1.0.0".to_owned()), d);
            },
            Err (err) => assert!(false, "should not have received error: {:?}", err),
        }
    }

    #[test]
    fn not_allow_empty_dependency_names() {
        let dep = schema_v1::Dependency::Simple("".to_owned());
        match convert_dependency(&dep) {
            Ok (result) => assert!(false, "should not have allowed empty dependency name: {:?}", result),
            Err (err) => assert_eq!(err, Error::V1(v1::Error::InvalidDependencyName("".to_owned())))
        }
    }

    #[test]
    fn warn_on_semver_in_version() {
        let dep = schema_v1::Dependency::Detailed (
            schema_v1::DetailedDependency {
                version: Some("foo+1.0.0".to_owned()),
                .. Default::default()
            }
        );
        match convert_dependency(&dep) {
            Ok ((d, w)) => {
                assert_eq!(v1::Dependency::DefaultRegistry("1.0.0".to_owned()), d);
                assert!(false, "d: {:?} - {:?}", d, w);
            },
            Err (err) => assert!(false, "should not have received error: {:?}", err),
        }
    }

    #[test]
    fn fail_to_convert_empty_dependency() {
        let dep = schema_v1::Dependency::Detailed (
            schema_v1::DetailedDependency {
                .. Default::default()
            }
        );
        match convert_dependency(&dep) {
            Ok(result) => assert!(false, "should not have converted: {:?}", result),
            Err(err) => assert!(false, "wrong error: {:?}", err),
        }
    }
}