//! TODO.
//!

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::serv_config::Args;

mod app_config;
mod serv_config;

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
