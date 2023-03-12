use serde::{Serialize, Serializer};

use crate::PackageKey;

impl Serialize for PackageKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.key.serialize(serializer)
    }
}
