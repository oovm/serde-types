mod methods;
mod third_party;

/// A case-insensitive, space normalized string, suitable as a package name.
///
/// # Examples
///
/// ```
/// # use package_key::InsensitiveKey;
/// let key1 = InsensitiveKey::new("package-name");
/// let key2 = InsensitiveKey::new("package_name");
/// assert_eq!(key1, key2);
/// ```
pub struct InsensitiveKey {
    key: String,
}
