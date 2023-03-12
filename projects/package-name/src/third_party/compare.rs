use crate::PackageKey;

impl PartialEq<String> for PackageKey {
    fn eq(&self, other: &str) -> bool {
        self.key == *other
    }
}
