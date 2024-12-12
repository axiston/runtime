//! [`RouteIndex`] and [`ServiceIndex`].

use derive_more::{Deref, DerefMut};
use ecow::EcoString;

/// Opaque and unique [`Service`] identifier.
///
/// [`Service`]: crate::routing::manifest::ServiceManifest
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deref, DerefMut)]
#[must_use = "indexes do nothing unless you serialize them"]
pub struct ServiceIndex {
    inner: EcoString,
}

impl ServiceIndex {
    /// Returns a new [`ServiceIndex`].
    #[inline]
    pub fn new<S: AsRef<str>>(inner: S) -> Self {
        let inner = EcoString::from(inner.as_ref());
        Self { inner }
    }

    /// Returns the underlying index.
    #[inline]
    pub fn into_inner(self) -> EcoString {
        self.inner.clone()
    }
}

/// Opaque and unique [`Route`] identifier.
///
/// [`Route`]: crate::routing::Route
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deref, DerefMut)]
#[must_use = "indexes do nothing unless you serialize them"]
pub struct RouteIndex {
    inner: EcoString,
}

impl RouteIndex {
    /// Returns a new [`RouteIndex`].
    #[inline]
    pub fn new<S: AsRef<str>>(inner: S) -> Self {
        let inner = EcoString::from(inner.as_ref());
        Self { inner }
    }

    /// Returns the underlying index.
    #[inline]
    pub fn into_inner(self) -> EcoString {
        self.inner.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::routing::RouteIndex;
    use crate::Result;

    #[test]
    fn index_from_string() -> Result<()> {
        let _ = RouteIndex::new("index");
        Ok(())
    }
}
