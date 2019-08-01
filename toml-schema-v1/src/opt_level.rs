use serde::de;
use serde::ser;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptLevel(pub String);

impl<'de> de::Deserialize<'de> for OptLevel {
    fn deserialize<D>(d: D) -> Result<OptLevel, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = OptLevel;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an optimization level")
            }

            fn visit_i64<E>(self, value: i64) -> Result<OptLevel, E>
            where
                E: de::Error,
            {
                Ok(OptLevel(value.to_string()))
            }

            fn visit_str<E>(self, value: &str) -> Result<OptLevel, E>
            where
                E: de::Error,
            {
                if value == "s" || value == "z" {
                    Ok(OptLevel(value.to_string()))
                } else {
                    Err(E::custom(format!(
                        "must be an integer, `z`, or `s`, \
                         but found: {}",
                        value
                    )))
                }
            }
        }

        d.deserialize_any(Visitor)
    }
}

impl ser::Serialize for OptLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.0.parse::<u32>() {
            Ok(n) => n.serialize(serializer),
            Err(_) => self.0.serialize(serializer),
        }
    }
}