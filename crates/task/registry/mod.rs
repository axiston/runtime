//! TODO. [`Registry`].
//!

use std::sync::{Arc, Mutex};

use crate::registry::action::{ActionManifest, ActionRequest, ActionResponse};
use crate::registry::trigger::{TriggerManifest, TriggerRequest, TriggerResponse};
use crate::routing::Router;

mod action;
mod custom_serde;
mod trigger;

/// TODO.
#[must_use = "routers do nothing unless you use them"]
#[derive(Debug, Default, Clone)]
pub struct Registry {
    inner: Arc<Mutex<RegistryInner>>,
}

#[derive(Debug, Default)]
struct RegistryInner {
    // registered_services: Router<ServiceIndex, ()>,
    registered_triggers: Router<TriggerRequest, TriggerResponse>,
    registered_actions: Router<ActionRequest, ActionResponse>,
}

impl Registry {
    /// Returns an empty [`Registry`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn register_action()
}

#[cfg(test)]
mod test {
    use crate::registry::Registry;
    use crate::Result;

    #[test]
    fn build_empty() -> Result<()> {
        let _ = Registry::new();
        Ok(())
    }
}
