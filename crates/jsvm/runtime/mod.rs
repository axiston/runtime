//! TODO.
//!

mod cert_provider;
mod deno_runtime;
mod filesystem;
mod module_loader;
mod permissions;
mod transpile;

use std::fmt;
use std::rc::Rc;

use tokio::runtime::Runtime as TokioRuntime;

use crate::runtime::deno_runtime::DenoRuntime;
pub use crate::runtime::permissions::{axis_permissions, MyPermission};

pub struct Jsvm {
    deno_runtime: DenoRuntime,
    tokio_runtime: TokioRuntime,
}

impl Jsvm {
    /// Returns a new [`Jsvm`].
    pub fn new(tokio_runtime: Rc<TokioRuntime>) -> Self {
        todo!()
    }

    // TODO: load modules
}

impl fmt::Debug for Jsvm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Jsvm").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::Result;

    #[test]
    fn build() -> Result<()> {
        Ok(())
    }
}
