//! TODO.
//!

pub use crate::service::config::{AppBuilder, AppConfig};

mod config;
mod instance;
mod registry;

/// Application state.
///
/// Used by [`handlers`].
///
/// [`handlers`]: crate::handler
#[derive(Debug, Clone)]
#[must_use = "state does nothing unless you use it"]
pub struct AppState {
    // runtime: Rc<Runtime>,
}

impl AppState {
    /// Creates a new [`AppState`].
    #[inline]
    pub fn new(config: AppConfig) -> Self {
        Self {}
    }
}
