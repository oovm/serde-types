use crate::InsensitiveKey;

impl Default for InsensitiveKey {
    fn default() -> Self {
        InsensitiveKey::new("")
    }
}

impl InsensitiveKey {
    /// Create a new key
    ///
    /// # Examples
    ///
    /// ```
    /// # use package_key::InsensitiveKey;
    /// let key = InsensitiveKey::new("package-name");
    /// ```
    pub fn new<S>(key: S) -> Self
    where
        S: AsRef<str>,
    {
        Self { key: norm_key(key.as_ref()) }
    }
    /// Make the key like a [&str](std::str).
    ///
    /// # Examples
    ///
    /// ```
    /// # use package_key::InsensitiveKey;
    /// let key = InsensitiveKey::new("package-name");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

fn norm_key(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '_' | '-' => out.push('_'),
            s if s.is_ascii_whitespace() => out.push('_'),
            _ => out.push(c.to_ascii_uppercase()),
        }
    }
    out
}
