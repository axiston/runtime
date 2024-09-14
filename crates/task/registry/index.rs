//! TODO.
//!

use std::ops::Deref;
use ecow::EcoString;
use tower_path::routing::index::UniqueIndex;

/// Opaque and unique entity identifier.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UnderlyingIndex {
    inner: EcoString
}

impl UnderlyingIndex {
    /// Returns a new [`UnderlyingIndex`].
    #[inline]
    pub fn new (inner: impl AsRef<str>) -> Self {
        let inner = EcoString::from(inner.as_ref());
        Self { inner }
    }
}

impl Deref  for UnderlyingIndex {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.as_str()
    }
}

/// TODO.
pub type ServiceIndex = UniqueIndex<UnderlyingIndex>;

/// TODO.
pub type TriggerIndex = UniqueIndex<UnderlyingIndex>;

/// TODO.
pub type ActionIndex = UniqueIndex<UnderlyingIndex>;
