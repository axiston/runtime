use deno_core::anyhow::Error;
use deno_core::{
    ModuleLoadResponse, ModuleLoader, ModuleSpecifier, RequestedModuleType, ResolutionKind,
};

pub struct MyModuleLoader {}

impl MyModuleLoader {
    /// Returns a new [`MyModuleLoader`].
    pub fn new() -> Self {
        Self {}
    }
}

impl ModuleLoader for MyModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, Error> {
        todo!()
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&ModuleSpecifier>,
        is_dyn_import: bool,
        requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        todo!()
    }
}
