//! Application state and dependency injection.

use axiston_rt_task::routing::layers::Layers;
use axiston_rt_task::routing::Router;

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::task_queue::TaskQueue;

mod app_config;
mod task_metrics;
mod task_queue;

/// Application state.
///
/// Used by [`handlers`].
///
/// [`handlers`]: crate::handler
#[derive(Debug, Clone)]
#[must_use = "state does nothing unless you use it"]
pub struct AppState {
    pub task_router: Router,
    pub task_queue: TaskQueue,
    // pub task_counter: TaskStatus,
    // pub runtime: Rc<Runtime>,
}

impl AppState {
    /// Creates a new [`AppState`].
    #[inline]
    pub fn new(config: AppConfig) -> Self {
        let layers = Layers::builder().build();

        Self {
            task_router: Router::new(layers),
            task_queue: TaskQueue::new(),
        }
    }
}
