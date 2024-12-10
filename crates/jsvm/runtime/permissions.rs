// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::borrow::Cow;
use std::path::Path;

use deno_core::error::AnyError;
use deno_core::url::Url;
use deno_fetch::FetchPermissions;
use deno_fs::FsPermissions;
use deno_io::fs::FsError;
use deno_net::NetPermissions;
use deno_permissions::NetDescriptor;
use deno_web::TimersPermission;
use deno_websocket::WebSocketPermissions;

/// TODO.
#[derive(Debug, Default, Clone)]
pub struct MyPermission {
    allow_net: bool,
    filter_net: Option<Vec<NetDescriptor>>,
}

deno_core::extension!(
    axis_permissions,
    options = {  allow_net: bool, filter_net: Option<Vec<NetDescriptor>> },
    state = |state, options| {
        state.put::<MyPermission>(
            MyPermission::new(options.allow_net, options.filter_net)
        );
    }
);

impl MyPermission {
    /// Returns a new [`MyPermission`].
    #[inline]
    pub fn new(allow_net: bool, filter_net: Option<Vec<NetDescriptor>>) -> Self {
        Self {
            allow_net,
            filter_net,
        }
    }
}

impl TimersPermission for MyPermission {
    #[inline]
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

impl FetchPermissions for MyPermission {
    fn check_net_url(&mut self, _url: &Url, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read(&mut self, _p: &Path, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }
}

impl NetPermissions for MyPermission {
    fn check_net<T: AsRef<str>>(
        &mut self,
        _host: &(T, Option<u16>),
        _api_name: &str,
    ) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_write(&mut self, _p: &Path, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }
}

impl WebSocketPermissions for MyPermission {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }
}

impl FsPermissions for MyPermission {
    fn check_open<'a>(
        &mut self,
        resolved: bool,
        read: bool,
        write: bool,
        path: &'a Path,
        api_name: &str,
    ) -> Result<Cow<'a, Path>, FsError> {
        Ok(Cow::Borrowed(path))
    }

    fn check_read(&mut self, path: &Path, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read_all(&mut self, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_read_blind(
        &mut self,
        p: &Path,
        display: &str,
        api_name: &str,
    ) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_write(&mut self, path: &Path, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_write_partial(&mut self, path: &Path, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_write_all(&mut self, api_name: &str) -> Result<(), AnyError> {
        Ok(())
    }

    fn check_write_blind(
        &mut self,
        p: &Path,
        display: &str,
        api_name: &str,
    ) -> Result<(), AnyError> {
        Ok(())
    }
}
