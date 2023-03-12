use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::PackageKey;

impl Hash for PackageKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<S> PartialEq<S> for PackageKey
where
    S: AsRef<str>,
{
    fn eq(&self, other: &S) -> bool {
        let rhs = PackageKey::new(other.as_ref());
        self.key.eq(&rhs.key)
    }
}

impl Eq for PackageKey {}

impl<S> PartialOrd<S> for PackageKey
where
    S: AsRef<str>,
{
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        let rhs = PackageKey::new(other.as_ref());
        self.key.partial_cmp(&rhs.key)
    }
}

impl Ord for PackageKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
