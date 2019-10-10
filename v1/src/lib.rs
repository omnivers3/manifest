// extern crate failure;
extern crate semver;
extern crate url;

mod dependency;
mod git_reference;
mod git_repository;
mod into_url;
mod manifest;
mod profile;
mod workspace;

pub use self::dependency::*;
pub use self::git_reference::*;
pub use self::git_repository::*;
pub use into_url::*;
pub use manifest::*;
pub use profile::*;
pub use workspace::*;

// use failure::{ Fail };
use std::fmt;
use std::path::{ PathBuf };

// #[derive(Debug, Fail, PartialEq)]
#[derive(Debug, PartialEq)]
pub enum Error {
    GitBaseUrlNotSupported(url::Url),
    InvalidDependencyName(String),
    InvalidPathUrl(PathBuf),
    InvalidStringUrl(String, url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::GitBaseUrlNotSupported(url) => {
                write!(f, "invalid url `{}`: cannot-be-a-base-URLs are not supported", url)
            },
            Error::InvalidDependencyName(key) => {
                write!(f, "invalid dependency name `{}`", key)
            },
            Error::InvalidPathUrl(path) => {
                write!(f, "invalid path url `{:?}`", path)
            },
            Error::InvalidStringUrl(url, err) => {
                write!(f, "invalid url `{:?}`: {}", url, err)
            },
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// From Cargo paths.rs
//
// pub fn normalize_path(path: &Path) -> PathBuf {
//     let mut components = path.components().peekable();
//     let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
//         components.next();
//         PathBuf::from(c.as_os_str())
//     } else {
//         PathBuf::new()
//     };

//     for component in components {
//         match component {
//             Component::Prefix(..) => unreachable!(),
//             Component::RootDir => {
//                 ret.push(component.as_os_str());
//             }
//             Component::CurDir => {}
//             Component::ParentDir => {
//                 ret.pop();
//             }
//             Component::Normal(c) => {
//                 ret.push(c);
//             }
//         }
//     }
//     ret
// }