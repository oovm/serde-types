use std::fmt::{Debug, Display, Formatter, Write};

use crate::InsensitiveKey;

impl Debug for InsensitiveKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.key, f)
    }
}

impl Display for InsensitiveKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.key.chars() {
            if c.is_whitespace() {
                f.write_char('-')?;
            }
            else {
                f.write_char(c.to_ascii_lowercase())?
            }
        }
        Ok(())
    }
}

impl AsRef<str> for InsensitiveKey {
    fn as_ref(&self) -> &str {
        &self.key
    }
}
