use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::InsensitiveKey;

impl Hash for InsensitiveKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<S> PartialEq<S> for InsensitiveKey
where
    S: AsRef<str>,
{
    fn eq(&self, other: &S) -> bool {
        let rhs = InsensitiveKey::new(other.as_ref());
        self.key.eq(&rhs.key)
    }
}

impl Eq for InsensitiveKey {}

impl<S> PartialOrd<S> for InsensitiveKey
where
    S: AsRef<str>,
{
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        let rhs = InsensitiveKey::new(other.as_ref());
        self.key.partial_cmp(&rhs.key)
    }
}

impl Ord for InsensitiveKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
