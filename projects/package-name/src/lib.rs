use serde_derive::{Deserialize, Serialize};

mod methods;
mod third_party;

#[derive(Serialize, Deserialize)]
pub struct PackageKey {
    key: String,
}

impl Default for PackageKey {
    fn default() -> Self {
        PackageKey::new("")
    }
}

impl PackageKey {
    pub fn new<S>(key: S) -> Self
    where
        S: AsRef<str>,
    {
        Self { key: norm_key(key.as_ref()) }
    }
    pub fn as_str(&self) -> &str {
        &self.key
    }
}

fn norm_key(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ' ' | '_' | '-' => out.push('_'),
            _ => out.push(c.to_ascii_uppercase()),
        }
    }
    out
}
