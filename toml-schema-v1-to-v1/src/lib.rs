extern crate failure;
extern crate semver;

extern crate omni_manifest_v1 as v1;
extern crate omni_manifest_toml_schema_v1 as schema_v1;

use failure::{ Fail };
// use semver::ReqParseError;
use semver::VersionReq;
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
    DependencyNameIsRequired,
    OneOfGitOrRegistry,
    OneOfGitOrPath,
    OneOfBranchTagOrRev,
}

impl fmt::Display for Constraint{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Constraint::DependencyNameIsRequired => "Dependeny name is required",
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
    VersionReq(semver::ReqParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Constraint(c) => write!(f, "Constraint violated: {}", c),
            Error::V1(err) => err.fmt(f),
            Error::VersionReq(err) => write!(f, "Version error: {}", err),
        }
    }
}

impl From<v1::Error> for Error {
    fn from(err: v1::Error) -> Error {
        Error::V1(err)
    }
}

pub type Result<T> = std::result::Result<(T, Option<Vec<Warning>>), Error>;

fn convert_detailed_dependency(src: schema_v1::DetailedDependency) -> Result<v1::Dependency> {
    let mut warnings = vec![];
    if src.version.is_none() && src.path.is_none() && src.git.is_none() {
        warnings.push(Warning::NoValidSources(src.to_owned()));
    }
    if let Some(v) = &src.version {
        if v.contains('+') {
            warnings.push(Warning::IgnoredMetadata(v.to_owned()))
        }
    }
    if src.git.is_none() {
        let keys: Vec<String> = [
            src.branch.as_ref().map(|_| GIT_KEY_BRANCH),
            src.tag.as_ref().map(|_| GIT_KEY_TAG),
            src.rev.as_ref().map(|_| GIT_KEY_REV)
        ].into_iter().filter_map(|k| k.map(|v|v.to_owned())).collect();
        if keys.len() > 0 {
            warnings.push(Warning::GitKeysIgnored(keys))
        }
    }
    match ( &src.git, &src.path, &src.registry ) {
        ( None, None, None ) => {
            VersionReq::parse(
                &src.version.unwrap_or_default()
            )
                .map(v1::Dependency::DefaultRegistry)
                .map_err(Error::VersionReq)
        },
        ( None, Some(path), Some(registry) ) => {
            warnings.push(Warning::RegistryIgnored(registry.to_owned()));
            Ok(v1::Dependency::LocalPath(path.into()))
        },
        ( None, Some(path), None) => {
            Ok(v1::Dependency::LocalPath(path.into()))
        },
        ( None, None, Some(registry)) => {
            Ok(v1::Dependency::CustomRegistry(registry.to_owned()))
        },
        ( Some(_), Some(_), _ ) => Err(Error::Constraint(Constraint::OneOfGitOrPath)),
        ( Some(_), None, Some(_) ) => Err(Error::Constraint(Constraint::OneOfGitOrRegistry)),
        ( Some(git), None, None ) => {
            let mut repo = v1::GitRepository::from_url_string(git.to_owned())?;
            match ( src.branch, src.tag, src.rev ) {
                ( None, None, None ) => {
                    Ok(repo.reference.to_owned())
                },
                ( Some(branch), None, None) => {
                    Ok(v1::GitReference::Branch(branch.to_owned()))
                },
                ( None, Some(tag), None) => {
                    Ok(v1::GitReference::Tag(tag.to_owned()))
                },
                ( None, None, Some(rev) ) => {
                    Ok(v1::GitReference::Rev(rev.to_owned()))
                },
                _ => {
                    Err(Error::Constraint(Constraint::OneOfBranchTagOrRev))
                }
            }.map(|reference| {
                repo.reference = reference;
                repo
            })
            .map(v1::Dependency::Git)
        },
    }.map(|d| {
        (d, if warnings.len() == 0 { None } else { Some(warnings) })
    })
}

fn validate_dependency((d, w): (v1::Dependency, Option<Vec<Warning>>)) -> Result<v1::Dependency> {
    // let mut warnings = w.unwrap_or_default();
    let warnings = w.unwrap_or_default();
    // match d {
    //     v1::Dependency::Git(_) => {}
    //     v1::Dependency::LocalPath(path) | v1::Dependency::Directory(path) => {

    //     },
    //     v1::Dependency::CustomRegistry(v) => {

    //     },
    //     v1::Dependency::
    // }
    // if d.version.len() == 0 {
    //     return Err(Error::Constraint(Constraint::DependencyNameIsRequired))
    // }
    let warnings = if warnings.len() == 0 { None } else { Some(warnings) };
    Ok((d, warnings))
}

pub fn convert_dependency(src: schema_v1::Dependency) -> Result<v1::Dependency> {
    match src {
        schema_v1::Dependency::Simple(ref value) => {
            VersionReq::parse(value)
                .map(v1::Dependency::DefaultRegistry)
                .map(|v| (v, None))
                .map_err(Error::VersionReq)
        },
        schema_v1::Dependency::Detailed(details) => convert_detailed_dependency(details),
    }
    .and_then(validate_dependency)
}

#[cfg(test)]
mod omni_toml_schema_v1_to_v1 {
    mod tests {
        use std::path::PathBuf;
        use std::str::FromStr;

        use v1;
        use schema_v1;
        use semver::VersionReq;

        use crate::{ convert_dependency, Error, Constraint, Warning };

        #[test]
        fn convert_named_dependency_without_warnings() {
            let dep = schema_v1::Dependency::Simple("1.0.0".to_owned());
            match convert_dependency(dep) {
                Ok ((d, w)) => {
                    assert_eq!(None, w);
                    assert_eq!(v1::Dependency::DefaultRegistry(VersionReq::parse("1.0.0").unwrap()), d);
                },
                Err (err) => assert!(false, "should not have received error: {:?}", err),
            }
        }

        #[test]
        fn default_simple_dependency_empty_version_to_major_wildcard() {
            let dep = schema_v1::Dependency::Simple("".to_owned());
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::DefaultRegistry(vr), _) => {
                            let vr: semver::VersionReq = vr;
                            assert!(vr.matches(&semver::Version::new(0, 0, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(0, 1, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(1, 0, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(2, 0, 0)), "invalid wildcard version mismatch");
                        },
                        _ => assert!(false, "wrong dependency, expected DefaultRegistry: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn warn_for_no_valid_sources() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (_, Some(w)) => {
                            assert!(w.iter().any(|w| {
                                match w {
                                    Warning::NoValidSources(_) => true,
                                    _ => false,
                                }
                            }), "should have contained NoValidSources in warnings: {:?}", w);
                        },
                        _ => assert!(false, "should have warnings: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn warn_on_semver_in_version() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    version: Some("1.0.0+foo".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok ((d, Some(w))) => {
                    assert_eq!(v1::Dependency::DefaultRegistry(VersionReq::parse("1.0.0").unwrap()), d);
                    assert!(w.iter().any(|w| {
                        match w {
                            Warning::IgnoredMetadata(_) => true,
                            _ => false,
                        }
                    }), "should have contained IgnoredMetadata in warnings: {:?}", w);
                },
                Ok ((d, None)) => assert!(false, "should include warning for ignored metadata: {:?}", d),
                Err (err) => assert!(false, "should not have received error: {:?}", err),
            }
        }

        #[test]
        fn warn_on_git_fields_without_git_source() {
            let expected_keys = ["branch".to_owned(), "rev".to_owned()];

            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    branch: Some("foo".to_owned()),
                    rev: Some("baz".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok ((d, Some(w))) => {
                    match d {
                        v1::Dependency::DefaultRegistry(_) => {},
                        _ => assert!(false, "should not have picked up a non-default registry"),
                    }
                    assert!(w.iter().any(|w| {
                        match w {
                            Warning::GitKeysIgnored(keys) => {
                                let eq = expected_keys.to_vec().eq(keys);
                                assert!(eq, "should contain keys used for git related entries: {:?} but was {:?}", expected_keys, keys);
                                true
                            },
                            _ => false,
                        }
                    }), "should have contained NoValidSources in warnings: {:?}", w);
                },
                Ok ((d, None)) => assert!(false, "should include warning for ignored metadata: {:?}", d),
                Err (err) => assert!(false, "should not have received error: {:?}", err),
            }
        }

        #[test]
        fn default_detailed_dependency_default_registry_version_to_major_wildcard() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::DefaultRegistry(vr), _) => {
                            assert!(vr.matches(&semver::Version::new(0, 0, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(0, 1, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(1, 0, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(2, 0, 0)), "invalid wildcard version mismatch");
                        },
                        _ => assert!(false, "wrong dependency, expected DefaultRegistry: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn detailed_dependency_default_registry_with_version_parsed() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    version: Some("^3.1".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::DefaultRegistry(vr), _) => {
                            assert!(!vr.matches(&semver::Version::new(0, 0, 0)), "invalid wildcard version mismatch");
                            assert!(!vr.matches(&semver::Version::new(1, 0, 0)), "invalid wildcard version mismatch");
                            assert!(vr.matches(&semver::Version::new(3, 1, 0)), "invalid wildcard version mismatch");
                        },
                        _ => assert!(false, "wrong dependency, expected DefaultRegistry: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn detailed_dependency_local_path_with_registry_ignored() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    registry: Some("ignored".to_owned()),
                    path: Some(".".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::LocalPath(path), Some(w)) => {
                            let expected_path = PathBuf::from_str(".").unwrap();
                            assert_eq!(expected_path, path);
                            assert!(w.iter().any(|w| {
                                match w {
                                    Warning::RegistryIgnored(key) => key == "ignored",
                                    _ => false,
                                }
                            }), "should have contained NoValidSources in warnings: {:?}", w);
                        },
                        (v1::Dependency::LocalPath(_), None) => assert!(false, "should have had warnings"),
                        _ => assert!(false, "wrong dependency, expected LocalPath: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn detailed_dependency_local_path_without_registry_ignored() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    path: Some(".".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::LocalPath(_), None) => {},
                        _ => assert!(false, "wrong dependency, expected LocalPath: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn detailed_dependency_registry() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    registry: Some("foo".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::CustomRegistry(key), _) => {
                            assert_eq!("foo".to_owned(), key);
                        },
                        _ => assert!(false, "wrong dependency, expected CustomRegistry: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }

        #[test]
        fn not_allow_git_and_registry() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    git: Some("bar".to_owned()),
                    registry: Some("foo".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (_) => assert!(false, "should have failed due to constraint"),
                Err (Error::Constraint(Constraint::OneOfGitOrRegistry)) => {},
                Err (err) => assert!(false, "expected OneOfGitOrRegistry: {:?}", err),
            }
        }

        #[test]
        fn not_allow_git_and_path() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    git: Some("bar".to_owned()),
                    path: Some(".".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (_) => assert!(false, "should have failed due to constraint"),
                Err (Error::Constraint(Constraint::OneOfGitOrPath)) => {},
                Err (err) => assert!(false, "expected OneOfGitOrPath: {:?}", err),
            }
        }

        #[test]
        fn not_allow_git_with_conflicting_keys() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    git: Some("http://foo".to_owned()),
                    branch: Some("bar".to_owned()),
                    tag: Some("baz".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (_) => assert!(false, "should have failed due to constraint"),
                Err (Error::Constraint(Constraint::OneOfBranchTagOrRev)) => {},
                Err (err) => assert!(false, "expected OneOfBranchTagOrRev: {:?}", err),
            }
        }

        #[test]
        fn detailed_dependency_git() {
            let dep = schema_v1::Dependency::Detailed (
                schema_v1::DetailedDependency {
                    git: Some("http://foo".to_owned()),
                    .. Default::default()
                }
            );
            match convert_dependency(dep) {
                Ok (result) => {
                    match result {
                        (v1::Dependency::Git(r), None) => {
                            assert_eq!(r.repo.to_string(), "http://foo/".to_owned());
                            assert_eq!(r.reference, v1::GitReference::Branch("master".to_owned()));
                        },
                        (v1::Dependency::Git(_), Some(w)) => assert!(false, "should not have had warnings but was: {:?}", w),
                        _ => assert!(false, "wrong dependency, expected LocalPath: {:?}", result),
                    }
                },
                Err (err) => assert!(false, "unexpected error: {}", err),
            }
        }
    }
}