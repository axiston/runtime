//! TODO.
//!

use std::hash::Hash;

/// TODO.
pub trait RouterIndex {}

/// TODO.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SegmentIndex<T = u32> {
    inner: T,
}

impl<T> SegmentIndex<T> {}

impl<T> RouterIndex for SegmentIndex<T> where T: Eq + Hash {}

/// TODO.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UniqueIndex<T = u32> {
    // TODO: P = ()
    inner: T,
}

impl<T> UniqueIndex<T> {}

impl<T> RouterIndex for UniqueIndex<T> where T: Eq + Hash {}

/// The default type for router indices.
pub type DefaultIx = UniqueIndex<u32>;
