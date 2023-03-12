use std::fmt::{Debug, Display, Formatter};

pub struct UniqueKey {
    key: String,
}

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

impl UniqueKey {
    pub fn new<S>(key: S) -> Self
        where
            S: Into<String>,
    {
        Self { key: key.as_ref().to_string() }
    }
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

impl Default for UniqueKey {
    fn default() -> Self {
        Self::new("")
    }
}

impl AsRef<str> for UniqueKey {
    fn as_ref(&self) -> &str {
        &self.key
    }
}
