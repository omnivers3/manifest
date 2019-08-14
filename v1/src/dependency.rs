use std::path::PathBuf;

use semver::VersionReq;

use crate::{ GitRepository };

#[derive(Clone, Debug, PartialEq)]
pub enum Dependency {
    // Versioned(String),
    Git(GitRepository),
    LocalPath(PathBuf),
    CustomRegistry(String), // TODO: Add locked (source_id.rs line 139)?
    DefaultRegistry(VersionReq), // Crates.io
    Directory(PathBuf),
}

pub enum ValidationError {
    InvalidDependencyName(String),
}

pub struct OptionVec<T>(Option<Vec<T>>);

impl<T> OptionVec<T> {
    pub fn new() -> Self {
        OptionVec(None)
    }

    pub fn push(&mut self, item: T) {
        if let Some(ref mut list) = self.0 {
            list.push(item);
        } else {
            self.0 = Some(vec![item]);
        }
    }
}

trait Validator {
    type Error;

    fn validate(&self) -> OptionVec<Self::Error>;
}

impl Validator for Dependency {
    type Error = ValidationError;

    fn validate(&self) -> OptionVec<ValidationError> {
        let mut errors = OptionVec::new();
        match self {
            // Dependency::Simple(value) => {
            //     if value.len() == 0 {
            //         errors.push(ValidationError::InvalidDependencyName(value.to_owned()));
            //     }
            // },
            Dependency::Git(_repo) => {},
            Dependency::LocalPath(_path) => {},
            Dependency::CustomRegistry(_key) => {},
            Dependency::DefaultRegistry(_version) => {},
            Dependency::Directory(_path) => {},
        }
        errors
    }
}