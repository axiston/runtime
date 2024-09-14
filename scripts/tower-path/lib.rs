#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! fn main() {}
//! ```

pub mod handler;
pub mod routing;
pub mod service;

// TODO: ServiceBuilderExt for TaskHandler.

// TODO: Move tower-task into its own repository.
// .github/dependabot.yaml,.github/workflows, rustfmt.toml
