use std::fmt::{Debug, Display, Formatter};

use crate::PackageKey;

impl Debug for PackageKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.key, f)
    }
}

impl Display for PackageKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.key, f)
    }
}

impl AsRef<str> for PackageKey {
    fn as_ref(&self) -> &str {
        &self.key
    }
}
