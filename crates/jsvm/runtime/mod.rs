//! TODO.
//!

mod machine;
mod permission;

use std::fmt;
use std::rc::Rc;

use deno_core::{JsRuntime, RuntimeOptions};
use tokio::runtime::Runtime as TokioRuntime;

pub struct Jsvm {
    inner: JsRuntime,
}

impl Jsvm {
    /// Returns a new [`Jsvm`].
    pub fn new(tokio_runtime: Rc<TokioRuntime>) -> Self {
        JsRuntime::init_platform(None, true);

        let options = RuntimeOptions {
            extensions: vec![],
            module_loader: None,
            extension_transpiler: None,
            ..RuntimeOptions::default()
        };

        let inner = JsRuntime::new(options);

        todo!()
    }

    // run
    // inspect
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
