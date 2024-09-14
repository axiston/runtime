use tower::Layer;

use crate::service::WithData;

/// `tower::`[`Layer`] that produces a [`WithData`] services.
#[derive(Debug, Default, Clone)]
pub struct WithDataLayer<M> {
    manifest: M,
}

impl<M> WithDataLayer<M> {
    /// Returns a new [`WithDataLayer`].
    #[inline]
    pub fn new(manifest: M) -> Self {
        Self { manifest }
    }
}

impl<S, M> Layer<S> for WithDataLayer<M>
where
    M: Clone,
{
    type Service = WithData<S, M>;

    #[inline]
    fn layer(&self, inner: S) -> Self::Service {
        WithData::new(inner, self.manifest.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::service::WithDataLayer;
    use crate::Result;

    #[test]
    fn layer() -> Result<()> {
        let _ = WithDataLayer::new(42u32);
        Ok(())
    }
}
