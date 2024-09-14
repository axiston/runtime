use serde::Deserialize;

/// Deserializable options for a [`tracing_*`] op.
///
/// [`tracing_*`]: crate::ext_tracing
#[derive(Debug, Default, Deserialize)]
#[must_use = "datatypes do nothing unless you deserialize them"]
pub struct TracingOptions {
    pub target: Option<String>,
}

impl TracingOptions {}

#[cfg(test)]
mod test {
    use crate::extension::trace::datatype::TracingOptions;
    use crate::extension::trace::Result;

    #[test]
    fn instance() -> Result<()> {
        let _ = TracingOptions::default();
        Ok(())
    }
}
