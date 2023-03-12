use std::fmt::{Debug, Display, Formatter};
use crate::UniqueKey;

impl Debug for UniqueKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.key, f)
    }
}

impl Display for UniqueKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.key, f)
    }
}
impl AsRef<str> for UniqueKey {
    fn as_ref(&self) -> &str {
        &self.key
    }
}