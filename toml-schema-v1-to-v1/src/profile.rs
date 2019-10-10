use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    V1(v1::Error),
    NestedProfileOverride,
    PanicNotAllowedInOverride,
    LtoNotAllowedInOverride,
    RPathNotAllowedInOverride,
    InvalidPanicSetting(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::V1(err) => err.fmt(f),
            Error::NestedProfileOverride => write!(f, "Profile overrides cannot be nested."),
            Error::PanicNotAllowedInOverride => write!(f, "`panic` may not be specified in a profile override."),
            Error::LtoNotAllowedInOverride => write!(f, "`lto` may not be specified in a profile override."),
            Error::RPathNotAllowedInOverride => write!(f, "`rpath` may not be specified in a profile override."),
            Error::InvalidPanicSetting(setting) => write!(f, "`panic` setting of `{}` is not a valid setting, must be `unwind` or `abort`", setting),
        }
    }
}

impl From<v1::Error> for Error {
    fn from(err: v1::Error) -> Error {
        Error::V1(err)
    }
}

pub type Result<T> = crate::ConvertResult<T, (), Error>;

fn validate_profile(src: &schema_v1::Profile) -> Option<Error> {
    if src.overrides.is_some() || src.build_override.is_some() {
        Some (Error::NestedProfileOverride)
    } else if src.panic.is_some() {
        Some (Error::PanicNotAllowedInOverride)
    } else if src.lto.is_some() {
        Some (Error::LtoNotAllowedInOverride)
    } else if src.rpath.is_some() {
        Some (Error::RPathNotAllowedInOverride)
    } else {
        None
    }
}

pub fn convert_profile(src: schema_v1::Profile) -> Result<v1::Profile> {
    if let Some(err) = src.build_override.and_then(|p| validate_profile(&p)) {
        return Err (err)
    }
    if let Some(err) = src.overrides.and_then(|o| {
        for profile in o.values() {
            if let Some(err) = validate_profile(profile) {
                return Some(err)
            }
        }
        None
    }) {
        return Err (err)
    }
    if let Some (panic) = src.panic {
        if panic != "unwind" && panic != "abort" {
            return Err(Error::InvalidPanicSetting(panic))
        }
    }
    Ok(( v1::Profile {}, None ))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{ convert_profile, Error };

    #[test]
    fn fail_to_convert_with_nested_build_override() {
        let invalid_nested = schema_v1::Profile {
            ..Default::default()
        };
        let profile_with_override = schema_v1::Profile {
            build_override: Some(Box::new(invalid_nested)),
            ..Default::default()
        };
        let p = schema_v1::Profile {
            build_override: Some(Box::new(profile_with_override)),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with nested override: {:?}", p),
            Err (err) => assert_eq!(Error::NestedProfileOverride, err),
        }
    }

    #[test]
    fn fail_to_convert_with_nested_overrides() {
        let invalid_nested = schema_v1::Profile {
            ..Default::default()
        };
        let profile_with_override = schema_v1::Profile {
            build_override: Some(Box::new(invalid_nested)),
            ..Default::default()
        };
        let mut map = BTreeMap::new();
        map.insert(schema_v1::ProfilePackageSpec::All, profile_with_override);
        let p = schema_v1::Profile {
            overrides: Some(map),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with nested override: {:?}", p),
            Err (err) => assert_eq!(Error::NestedProfileOverride, err),
        }
    }

    #[test]
    fn fail_to_convert_with_panic_in_override() {
        let sub_profile = schema_v1::Profile {
            panic: Some("unwind".to_owned()),
            ..Default::default()
        };
        let p = schema_v1::Profile {
            build_override: Some(Box::new(sub_profile)),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with panic in override: {:?}", p),
            Err (err) => assert_eq!(Error::PanicNotAllowedInOverride, err),
        }
    }

    #[test]
    fn fail_to_convert_with_lto_in_override() {
        let sub_profile = schema_v1::Profile {
            lto: Some(schema_v1::string_or_bool::StringOrBool::String("foo".to_owned())),
            ..Default::default()
        };
        let p = schema_v1::Profile {
            build_override: Some(Box::new(sub_profile)),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with lto in override: {:?}", p),
            Err (err) => assert_eq!(Error::LtoNotAllowedInOverride, err),
        }
    }

    #[test]
    fn fail_to_convert_with_rpath_in_override() {
        let sub_profile = schema_v1::Profile {
            rpath: Some(false),
            ..Default::default()
        };
        let p = schema_v1::Profile {
            build_override: Some(Box::new(sub_profile)),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with rpath in override: {:?}", p),
            Err (err) => assert_eq!(Error::RPathNotAllowedInOverride, err),
        }
    }

    #[test]
    fn fail_to_convert_invalid_panic() {
        let p = schema_v1::Profile {
            panic: Some("foo".to_owned()),
            ..Default::default()
        };
        match convert_profile(p) {
            Ok (p) => assert!(false, "should not convert with invalid panic setting: {:?}", p),
            Err (err) => assert_eq!(Error::InvalidPanicSetting("foo".to_owned()), err),
        }
    }
}