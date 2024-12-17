use clap::Parser;

/// Command-line arguments.
#[derive(Debug, Parser)]
pub struct Args {
    /// Bound server port.
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

impl Args {
    /// Returns a new [`Args`].
    #[inline]
    pub fn new() -> Self {
        Self::parse()
    }
}
