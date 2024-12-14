//! All `tonic::`[`Server`]s with related handlers.
//!
//! [`Server`]: tonic::transport::Server

use std::borrow::Cow;

use derive_more::From;
use tonic::{Code, Status};

pub use crate::handler::instance::InstanceService;
pub use crate::handler::registry::RegistryService;

mod instance;
mod registry;

/// The error type for [`Server`]s.
///
/// [`Server`]: tonic::transport::Server
#[derive(Debug, Default, From)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns a new [`Error`].
    #[inline]
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    /// Returns a new [`Status`].
    pub fn into_status(self) -> Status {
        todo!()
    }
}

impl From<Error> for Status {
    #[inline]
    fn from(value: Error) -> Self {
        value.into_status()
    }
}

/// Comprehensive list of all possible [`Error`]s.
#[derive(Debug, Default, Clone, Copy)]
#[must_use = "errors do nothing unless you use them"]
pub enum ErrorKind {
    #[default]
    Unknown,
    Aborted,
}

impl ErrorKind {
    /// Explicitly converts into the [`Error`].
    #[inline]
    pub fn into_error(self) -> Error {
        self.into()
    }

    /// Returns a new [`ErrorRepr`].
    pub fn into_repr(self) -> ErrorRepr<'static> {
        match self {
            ErrorKind::Unknown => ErrorRepr::INTERNAL_SERVICE_ERROR,
            ErrorKind::Aborted => ErrorRepr::SERVICE_WAS_ABORTED,
        }
    }

    /// Returns a new [`Status`].
    pub fn into_status(self) -> Status {
        self.into_repr().into_status()
    }
}

/// Internal representation of a serialized [`Error`] response.
#[derive(Debug, Clone)]
#[must_use = "errors do nothing unless you use them"]
struct ErrorRepr<'a> {
    pub message: Cow<'a, str>,
    pub code: Code,
}

impl<'a> ErrorRepr<'a> {
    const INTERNAL_SERVICE_ERROR: Self = Self::new(
        "Internal service error. Unknown underlying error or panic.",
        Code::Unknown,
    );

    const SERVICE_WAS_ABORTED: Self = Self::new(
        "Request processing was aborted by either client or server.",
        Code::Unknown,
    );

    /// Returns a new [`ErrorRepr`].
    #[inline]
    pub const fn new(message: &'a str, code: Code) -> Self {
        Self {
            message: Cow::Borrowed(message),
            code,
        }
    }

    /// Returns a new [`Status`].
    #[inline]
    pub fn into_status(self) -> Status {
        Status::new(self.code, self.message)
    }
}

impl From<ErrorRepr<'_>> for Status {
    #[inline]
    fn from(value: ErrorRepr<'_>) -> Self {
        value.into_status()
    }
}

/// A specialized [`Result`] type for the [`Error`] type.
///
/// Used by [`Server`]s.
///
/// [`Result`]: std::result::Result
/// [`Server`]: tonic::transport::Server
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use crate::handler::{Error, ErrorKind};

    #[test]
    fn build_default_error() {
        let error = Error::default();
        let _ = error.into_status();
    }

    #[test]
    fn build_error_kind() {
        let error = Error::new(ErrorKind::default());
        let _ = error.into_status();
    }
}
