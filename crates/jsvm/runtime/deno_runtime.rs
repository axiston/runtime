// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::rc::Rc;
use std::sync::{Arc, LazyLock};

use deno_core::anyhow::{self, anyhow};
use deno_core::{JsRuntime, ModuleCodeString, ModuleLoader, RuntimeOptions};
use deno_http::DefaultHttpPropertyExtractor;
use deno_io::Stdio;
use deno_tls::rustls::RootCertStore;
use deno_tls::RootCertStoreProvider;

use crate::runtime::cert_provider::MyRootCertStoreProvider;
use crate::runtime::filesystem::{CompileFs, FileBackedVfs, StaticFs};
use crate::runtime::module_loader::MyModuleLoader;
use crate::runtime::{axis_permissions, MyPermission};

/// Header value of the `User-Agent` key.
static AXISTON_UA: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Deno/{} (Variant; Axiston/{})",
        option_env!("CARGO_PKG_VERSION").unwrap_or("1.0"),
        env!("CARGO_PKG_VERSION"),
    )
});

#[ctor::ctor]
fn init_v8_platform() {
    // Initialize V8 flags.
    let v8_flags = std::env::var("V8_FLAGS").unwrap_or_default();

    if !v8_flags.is_empty() {
        let flags: Vec<_> = v8_flags.split(' ').map(|x| x.to_owned()).collect();
        let flags = deno_core::v8_set_flags(flags).iter();
        for flag in flags.filter(|flag| flag.is_empty()) {
            // TODO: Setup console tracing_subscriber.
            tracing::error!(target: "v8:init", flag = flag, "flag unrecognized");
        }
    }

    // NOTE(denoland/deno/20495): Due to new PKU feature introduced in V8 11.6 we need
    // to initialize the V8 platform on a parent thread of all threads that will spawn
    // V8 isolates.
    JsRuntime::init_platform(None, false);
}

pub struct DenoRuntime {
    js_runtime: JsRuntime,
}

impl DenoRuntime {
    async fn new(maybe_seed: Option<u64>) -> Self {
        let mut root_cert_store = RootCertStore::empty();
        let root_cert_store_provider: Arc<dyn RootCertStoreProvider> =
            Arc::new(MyRootCertStoreProvider::new(root_cert_store.clone()));
        let mut stdio = Some(Stdio::default());

        // let op_fs = {
        //     if is_user_worker {
        //         Arc::new(StaticFs::new(
        //             static_files,
        //             base_dir_path,
        //             vfs_path,
        //             vfs,
        //             npm_snapshot,
        //         )) as Arc<dyn deno_fs::FileSystem>
        //     } else {
        //         Arc::new(CompileFs::from_rc(vfs)) as Arc<dyn deno_fs::FileSystem>
        //     }
        // };

        let file_backed_vfs = FileBackedVfs::new();
        let op_fs = Arc::new(CompileFs::new(file_backed_vfs)) as Arc<dyn deno_fs::FileSystem>;

        let extensions = vec![
            axis_permissions::init_ops(true, None),
            deno_webidl::deno_webidl::init_ops(),
            deno_console::deno_console::init_ops(),
            deno_url::deno_url::init_ops(),
            deno_web::deno_web::init_ops::<MyPermission>(
                Arc::new(deno_web::BlobStore::default()),
                None,
            ),
            deno_webgpu::deno_webgpu::init_ops(),
            deno_canvas::deno_canvas::init_ops(),
            deno_fetch::deno_fetch::init_ops::<MyPermission>(deno_fetch::Options {
                user_agent: AXISTON_UA.clone(),
                root_cert_store_provider: Some(root_cert_store_provider.clone()),
                ..Default::default()
            }),
            deno_websocket::deno_websocket::init_ops::<MyPermission>(
                AXISTON_UA.clone(),
                Some(root_cert_store_provider.clone()),
                None,
            ),
            deno_crypto::deno_crypto::init_ops(maybe_seed),
            deno_net::deno_net::init_ops::<MyPermission>(Some(root_cert_store_provider), None),
            deno_tls::deno_tls::init_ops(),
            deno_http::deno_http::init_ops::<DefaultHttpPropertyExtractor>(),
            deno_io::deno_io::init_ops(stdio),
            deno_fs::deno_fs::init_ops::<MyPermission>(op_fs),
            // deno_node::init_ops::<Permissions>(Some(node_resolver), Some(npm_resolver), op_fs),
        ];

        let module_loader = Rc::new(MyModuleLoader::new()) as Rc<dyn ModuleLoader>;

        let runtime_options = RuntimeOptions {
            extensions,
            is_main: true,
            module_loader: Some(module_loader),
            ..RuntimeOptions::default()
        };

        let mut js_runtime = JsRuntime::new(runtime_options);

        let bootstrap_script = "";
        let bootstrap_module = ModuleCodeString::from(bootstrap_script);

        js_runtime
            .execute_script(deno_core::located_script_name!(), bootstrap_module)
            .expect("Failed to execute bootstrap script");

        Self { js_runtime }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn inspector(&self) {
        todo!()
    }

    /// Returns a new [`DenoRuntimeBuilder`].
    #[inline]
    pub fn builder() -> DenoRuntimeBuilder {
        DenoRuntimeBuilder::new()
    }
}

/// [`DenoRuntime`] builder.
#[must_use = "runtime does nothing unless you use itt"]
#[derive(Debug, Default)]
pub struct DenoRuntimeBuilder {
    maybe_seed: Option<u64>,
}

impl DenoRuntimeBuilder {
    /// Returns a new [`DenoRuntimeBuilder`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or overrides the initial seed for the crypto extension.
    pub fn with_crypto_seed(mut self, seed: u64) -> Self {
        self.maybe_seed = Some(seed);
        self
    }

    pub fn build(self) -> DenoRuntime {
        let extensions = vec![];

        let options = RuntimeOptions {
            extensions,
            ..RuntimeOptions::default()
        };

        todo!()
    }
}

#[cfg(test)]
mod test {}
