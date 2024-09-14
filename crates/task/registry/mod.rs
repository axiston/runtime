//! TODO.
//!

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use tower_path::routing::Router;

use crate::registry::handler::{ActionHandler, TriggerHandler};
use crate::registry::index::{ActionIndex, ServiceIndex, TriggerIndex};
use crate::Result;

mod handler;
mod index;
mod cache;

/// TODO.
#[derive(Debug, Default, Clone)]
pub struct Registry {
    inner: Arc<Mutex<RegistryInner>>,
}

#[derive(Debug, Default)]
struct RegistryInner {
    services: HashMap<ServiceIndex, ()>,
    triggers: Router<TriggerHandler, TriggerIndex>,
    actions: Router<ActionHandler, ActionIndex>,
}

impl Registry {
    /// Returns an empty [`Registry`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO.
    pub fn register_service(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub fn register_trigger(&self) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub fn register_action(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {}
