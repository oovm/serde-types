use serde::{Serialize, Serializer};

use crate::InsensitiveKey;

impl Serialize for InsensitiveKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.key.serialize(serializer)
    }
}
