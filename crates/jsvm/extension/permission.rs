use deno_permissions::NetDescriptor;

/// TODO.
#[derive(Debug, Default, Clone)]
pub struct MyPermission {
    allow_net: bool,
    filter_net: Vec<NetDescriptor>,
}

impl MyPermission {
    /// Returns a new [`MyPermission`].
    #[inline]
    pub fn new(allow_net: bool, filter_net: Vec<NetDescriptor>) -> Self {
        Self {
            allow_net,
            filter_net,
        }
    }
}
