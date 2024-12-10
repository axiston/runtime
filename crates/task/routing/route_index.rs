use std::ops::{Deref, DerefMut};

/// Opaque and unique identifier.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RouteIndex<T = ()> {
    inner: T,
}

impl<T> RouteIndex<T> {
    /// Returns a new [`RouteIndex`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    /// Returns the underlying index.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Deref for RouteIndex<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RouteIndex<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::Result;

    #[test]
    fn index_from_string() -> Result<()> {
        Ok(())
    }
}
