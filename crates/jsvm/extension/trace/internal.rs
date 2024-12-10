use tracing::{debug, error, info, trace, warn, Level};

use crate::extension::trace::datatype::TracingOptions;
use crate::extension::trace::Result;

/// TODO.
pub fn emit_op_tracing_event(
    message: &str,
    level: Level,
    options: Option<TracingOptions>,
) -> Result<()> {
    let options = options.unwrap_or_default();
    let target = options.target.unwrap_or_default();

    match level {
        Level::TRACE => trace!(message),
        Level::DEBUG => debug!(message),
        Level::INFO => info!(message),
        Level::WARN => warn!(message),
        Level::ERROR => error!(message),
    };

    Ok(())
}
