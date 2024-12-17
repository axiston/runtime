use deno_core::error::AnyError;
use deno_tls::rustls::RootCertStore;
use deno_tls::RootCertStoreProvider;

#[derive(Debug, Clone)]
pub struct MyRootCertStoreProvider {
    root_cert_store: RootCertStore,
}

impl MyRootCertStoreProvider {
    /// Returns a new [`MyRootCertStoreProvider`].
    #[inline]
    pub fn new(root_cert_store: RootCertStore) -> Self {
        Self { root_cert_store }
    }
}

impl RootCertStoreProvider for MyRootCertStoreProvider {
    #[inline]
    fn get_or_try_init(&self) -> Result<&RootCertStore, AnyError> {
        Ok(&self.root_cert_store)
    }
}
