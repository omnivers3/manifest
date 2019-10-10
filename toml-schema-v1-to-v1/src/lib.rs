// extern crate failure;
extern crate semver;

extern crate omni_manifest_v1 as v1;
extern crate omni_manifest_toml_schema_v1 as schema_v1;

use std::fmt;

pub mod dependency;
pub mod profile;

// use dependency::{ convert_dependency };
// use profile::{ convert_profile };

pub type OptionVec<T> = Option<Vec<T>>;

pub type ConvertResult<T, W, E> = std::result::Result<(T, OptionVec<W>), E>;

#[derive(Debug, PartialEq)]
pub enum Warning {
    None,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::None => write!(f, "none"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Error {
    None (schema_v1::Manifest),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::None (manifest) => write!(f, "None: {:?}", manifest),
        }
    }
}

pub type Result<T> = ConvertResult<T, Warning, Error>;

pub fn convert(src: &schema_v1::Manifest) -> Result<v1::Manifest> {
    if let Some(workspace) = &src.workspace {
        // TODO: validate property values
        let workspace = v1::Workspace {
            members: workspace.members.clone(),
            default_members: workspace.default_members.clone(),
            exclude: workspace.exclude.clone(),
        };
        return Ok((v1::Manifest::Workspace(workspace), None))
    }

    // check for workspace in schema and return that enum if found...
    // let src_copy: schema_v1::Manifest = src.clone();
    Err ( Error::None(src.to_owned()) )
}

#[cfg(test)]
mod tests {
    // use v1;
    use schema_v1;

    use super::{ convert, Error, Warning };

    #[test]
    fn convert_workspace() {
        let man = schema_v1::Manifest {
            .. Default::default()
        };
        match convert(&man) {
            Err (err) => assert!(false, "unepxected error: {:?}", err),
            Ok ((v1::Manifest::Project, _)) => assert!(false, "should have been a workspace enum"),
            Ok ((v1::Manifest::Workspace(workspace), _)) => {
                    assert!(false, "foo: {:?}", workspace);
            }
        }
    }
}