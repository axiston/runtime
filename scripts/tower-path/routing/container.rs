use std::collections::HashMap;
use std::hash::Hash;

/// TODO.
pub trait RouterContainer<I, S> {
    /// Inserts an index-handler pair into the [`RouterContainer`].
    fn route(&mut self, index: I, service: S) -> Option<S>;

    /// Removes an index from the [`RouterContainer`].
    fn forget(&mut self, index: I) -> Option<S>;
}

impl<I, S> RouterContainer<I, S> for () {
    #[inline]
    fn route(&mut self, _index: I, _service: S) -> Option<S> {
        None
    }

    #[inline]
    fn forget(&mut self, _index: I) -> Option<S> {
        None
    }
}

impl<I, S> RouterContainer<I, S> for HashMap<I, S>
where
    I: Eq + Hash,
    S: Clone,
{
    #[inline]
    fn route(&mut self, index: I, service: S) -> Option<S> {
        self.insert(index, service)
    }

    #[inline]
    fn forget(&mut self, index: I) -> Option<S> {
       self.remove(&index)
    }
}

impl<I, S> RouterContainer<I, S> for Vec<S>
where
    I: Into<usize>,
    S: Clone,
{
    #[inline]
    fn route(&mut self, index: I, service: S) -> Option<S> {
        todo!()
    }

    #[inline]
    fn forget(&mut self, index: I) -> Option<S> {
        todo!()
    }
}
