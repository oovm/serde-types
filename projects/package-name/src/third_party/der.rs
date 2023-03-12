use serde::{de::Visitor, Deserialize, Deserializer};

use crate::InsensitiveKey;
use serde::de::Error;
struct KeyVisitor;

impl<'de> Deserialize<'de> for InsensitiveKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(KeyVisitor)
    }
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        *place = deserializer.deserialize_str(KeyVisitor)?;
        Ok(())
    }
}

impl<'de> Visitor<'de> for KeyVisitor {
    type Value = InsensitiveKey;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expect a `PackageKey` string")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(InsensitiveKey::new(v))
    }
}
