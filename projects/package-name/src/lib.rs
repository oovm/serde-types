mod third_party;
use std::fmt::{Debug, Display, Formatter};

pub struct UniqueKey {
    key: String,
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

