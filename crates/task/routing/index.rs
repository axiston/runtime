use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use deunicode::deunicode_with_tofu;
use ecow::EcoString;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Opaque and unique [`TaskHandler`] identifier.
///
/// [`TaskHandler`]: crate::handler::TaskHandler
#[must_use = "ids do nothing unless you use them"]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Index<T = ()> {
    inner: EcoString,
    marker: PhantomData<T>,
}

impl<T> Index<T> {
    /// Returns a new [`Index`].
    #[inline]
    pub fn new(inner: &str) -> Self {
        Self {
            inner: inner.into(),
            marker: PhantomData,
        }
    }

    /// Parses a new [`Index`].
    #[inline]
    pub fn parse(index: impl AsRef<str>) -> Result<Self> {
        index.as_ref().parse::<Index<T>>()
    }

    /// Extracts a string slice containing the entire id.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

impl<T> FromStr for Index<T> {
    type Err = Error;

    fn from_str(inner: &str) -> Result<Self, Self::Err> {
        let tofu = "\u{FFFD}";
        let deunicoded = deunicode_with_tofu(inner, tofu);
        if deunicoded.contains(tofu) {
            // return Err(Error::InvalidId(deunicoded));
            panic!("invalid id")
        }

        Ok(deunicoded.to_lowercase().replace(' ', "-").into())
    }
}

impl<T> Clone for Index<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> From<String> for Index<T> {
    #[inline]
    fn from(value: String) -> Self {
        Self::new(value.as_str())
    }
}

impl<T> From<&str> for Index<T> {
    #[inline]
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl<T> Hash for Index<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T> PartialEq<Self> for Index<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T> Eq for Index<T> {}

impl<T> Deref for Index<T> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<T> fmt::Display for Index<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<T> fmt::Debug for Index<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

#[cfg(test)]
mod test {
    use crate::routing::Index;
    use crate::Result;

    #[test]
    pub fn instance() -> Result<()> {
        let id = Index::<()>::new("service-entity");
        assert_eq!(id.as_str(), "service-entity");
        Ok(())
    }

    #[test]
    pub fn parse() -> Result<()> {
        let id = "Service Entity".parse::<Index<()>>()?;
        assert_eq!(id.as_str(), "service-entity");
        Ok(())
    }
}
