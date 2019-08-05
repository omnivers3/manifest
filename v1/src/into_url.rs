use std::path::{Path, PathBuf};

use url::Url;

use crate::{ Error, Result };

/// A type that can be converted to a Url
pub trait IntoUrl {
    /// Performs the conversion
    fn into_url(self) -> Result<Url>;
}

impl<'a> IntoUrl for &'a str {
    fn into_url(self) -> Result<Url> {
        Url::parse(self)
            .map_err(|err| Error::InvalidStringUrl(self.to_owned(), err))
    }
}

impl<'a> IntoUrl for String {
    fn into_url(self) -> Result<Url> {
        Url::parse(&self)
            .map_err(|err| Error::InvalidStringUrl(self, err))
    }
}

impl<'a> IntoUrl for &'a Path {
    fn into_url(self) -> Result<Url> {
        Url::from_file_path(self)
            .map_err(|()| Error::InvalidPathUrl(self.to_owned()))
    }
}

impl<'a> IntoUrl for &'a PathBuf {
    fn into_url(self) -> Result<Url> {
        self.as_path().into_url()
    }
}
