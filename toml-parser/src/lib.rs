extern crate failure;
extern crate serde;
extern crate serde_ignored;
extern crate omni_manifest_toml_schema_v1 as schema;

use failure::{ Fail };
use serde::de::Deserialize;
use std::fmt;

#[derive(Debug, Fail)]
pub enum Error {
    DeserializerError(toml::de::Error),
    FailedToParseToml(String),
    UnusedKeys(Vec<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DeserializerError(err) => {
                write!(f, "Failed to Deserialize TOML into Manifest: {}", err)
            }
            Error::FailedToParseToml(err) => {
                write!(f, "Failed to parse toml: {}", err)
            },
            Error::UnusedKeys(keys) => {
                let mut r = write!(f, "Unused keys:");
                for key in keys {
                    r = r.and_then(|_| write!(f, "\n- {}", key));
                }
                r
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn parse_err<S: Into<String>>(value: S) -> Error {
    Error::FailedToParseToml(value.into())
}

pub const TOML_WITHOUT_NEWLINES: &'static str =
r#"The TOML spec requires newlines after table definitions (e.g., `[a] b = 1` is
invalid), but this file has a table header which does not have a newline after
it. A newline needs to be added in order to parse this file.
'cargo' will also reflect this warning as a hard error in the future."#;

/// Deserializer which allows a less restrictive TOML parser option; Used
/// to determine if previously allowed syntax "newlines after a table" is
/// represented in the provided manifest
fn deserialize_toml_without_newlines(data: &str) -> Option<Error> {
    let mut parser = toml::de::Deserializer::new(data);
    parser.set_require_newline_after_table(false);
    toml::Value::deserialize(&mut parser)
        .ok()
        .map(|_| parse_err(TOML_WITHOUT_NEWLINES))
}

/// Attempts to parse the supplied string as TOML and normalizes the result
/// into a failure::Error
pub fn parse_toml(data: &str) -> Result<toml::Value> {
    data
        .parse()
        .map_err(|e| parse_err(format!("{:?}", e)))
}

/// Attempts to parse the supplied string as a Rust Manifest TOML which`
/// normalizes the result to a failure::Error and upcasts a couple
/// specific error types from prior Cargo versions
pub fn parse_cargo_toml(data: &str) -> Result<toml::Value> {
    parse_toml(data)
        .map_err(|e| deserialize_toml_without_newlines(data).unwrap_or(e))
}

fn stringify_serde_ignored_path(dst: &mut String, path: &serde_ignored::Path<'_>) {
    use serde_ignored::Path;

    match *path {
        Path::Root => {}
        Path::Seq { parent, index } => {
            stringify_serde_ignored_path(dst, parent);
            if !dst.is_empty() {
                dst.push('.');
            }
            dst.push_str(&index.to_string());
        }
        Path::Map { parent, ref key } => {
            stringify_serde_ignored_path(dst, parent);
            if !dst.is_empty() {
                dst.push('.');
            }
            dst.push_str(key);
        }
        Path::Some { parent }
        | Path::NewtypeVariant { parent }
        | Path::NewtypeStruct { parent } => stringify_serde_ignored_path(dst, parent),
    }
}

pub fn parse_v1(data: &str) -> Result<schema::Manifest> {
    parse_cargo_toml(data)
        .and_then(|toml| {
            let mut ignored = Vec::new();
            serde_ignored::deserialize(toml, |path| {
                let mut path_str = String::new();
                stringify_serde_ignored_path(&mut path_str, &path);
                ignored.push(path_str);
            })
            .map_err(Error::DeserializerError)
            .and_then(|m| {
                if ignored.len() == 0 {
                    Ok(m)
                } else {
                    Err(Error::UnusedKeys(ignored))
                }
            })
        })
}

#[cfg(test)]
mod omni_toml_parser_tests {
    extern crate semver;

    use crate::{ Error, TOML_WITHOUT_NEWLINES };
    use crate::{ parse_cargo_toml, parse_toml, parse_v1 };

    #[test]
    fn parse_valid_minimal_manifest() {
        let result = parse_cargo_toml(r#"
            [package]
            name = "foo"
            version = "0.1.0"
        "#);

        println!("Result:\n{:?}\n", result);

        assert!(result.is_ok(), "should have parsed valid toml");
    }

    
    #[test]
    fn wrap_invalid_toml_error() {
        // No need to test the capability of the TOML parser just that we
        // properly wrap errors it raises into the local Error type
        match parse_toml(r#"
            [package
        "#) {
            Ok (_) => assert!(false, "should not have parsed invalid toml"),
            Err (err) => match err {
                Error::FailedToParseToml (_) => {},
                _ => assert!(false, "should have wrapped response in correct local error")
            }
        }
    }

    #[test]
    fn fail_with_custom_error_for_missing_newline_after_header() {
        // Don't test invalid TOML parsing as that behavior is covered by the
        // library being leveraged, focus on features specific to this crate
        match parse_cargo_toml(r#"[header]foo = 1"#) {
            Ok(_) => assert!(false, "should have failed to parse toml with customer error message"),
            Err(err) => assert_eq!(
                format!("{:?}", err),
                format!("{:?}", Error::FailedToParseToml(TOML_WITHOUT_NEWLINES.to_owned()))
            ),
        }
    }

    #[test]
    fn fail_to_parse_with_unused_toml_keys() {
        match parse_v1(r#"
            invalid = 1
        "#) {
            Ok(_) => assert!(false, "should have failed to parse due to presence of unused key 'invalid'"),
            Err(err) => {
                match err {
                    Error::UnusedKeys(keys) => {
                        assert_eq!(1, keys.len());
                        assert_eq!("invalid", keys[0]);
                    },
                    _ => assert!(false, "invalid error type {}", err),
                }
            }
        }
    }

    #[test]
    fn parse_empty_toml_to_all_none_manifest_struct() {
        match parse_v1(r#"
        "#) {
            Ok(m) => {
                assert_eq!(None, m.cargo_features);
                assert_eq!(None, m.package);
                assert_eq!(None, m.package);
                assert_eq!(None, m.profile);
                assert_eq!(None, m.lib);
                assert_eq!(None, m.bin);
                assert_eq!(None, m.example);
                assert_eq!(None, m.test);
                assert_eq!(None, m.bench);
                assert_eq!(None, m.dependencies);
                assert_eq!(None, m.dev_dependencies);
                assert_eq!(None, m.build_dependencies);
                assert_eq!(None, m.features);
                assert_eq!(None, m.target);
                assert_eq!(None, m.replace);
                assert_eq!(None, m.patch);
                assert_eq!(None, m.workspace);
                assert_eq!(None, m.badges);
            },
            Err(err) => assert!(false, "should have parsed successfully but instead:\n{}", err),
        }
    }

    #[test]
    fn fail_to_parse_invalid_package() {
        match parse_v1(r#"
        package = 1
        "#) {
            Ok(_) => assert!(false, "should not have parsed invalid package structure"),
            Err(err) => {
                match err {
                    Error::DeserializerError(_) => {},
                    _ => assert!(false, "should have gotten FailedToParseToml but was:\n{}", err),
                }
            }
        }
    }

    #[test]
    fn fail_to_parse_package_with_missing_required_fields() {
        match parse_v1(r#"
        [package]
        "#) {
            Ok(_) => assert!(false, "should not have parsed invalid package structure"),
            Err(err) => {
                match err {
                    Error::DeserializerError(_) => {},
                    _ => assert!(false, "should have gotten FailedToParseToml but was:\n{}", err),
                }
            }
        }
    }

    #[test]
    fn parse_valid_package() {
        match parse_v1(r#"
        [package]
        name = "foo"
        version = "1.0.0"
        "#) {
            Ok(m) => {
                match m.package {
                    None => assert!(false, "should have a value for manifest package field but is:\n{:?}", m),
                    Some (package) => {
                        assert_eq!("foo", package.name);
                        assert_eq!(semver::Version::parse("1.0.0").unwrap(), package.version);
                    }
                }
            },
            Err(err) => assert!(false, "should have parsed valid minimum package structure but was:\n{}", err),
        }
    }
}
