// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::path::{Path, PathBuf};

use deno_ast::ModuleSpecifier;
use deno_core::anyhow::anyhow;
use deno_core::error::AnyError;
use deno_core::serde_json;
use serde::{Deserialize, Serialize};

use crate::runtime::transpile::DiskCache;

#[derive(Debug, Deserialize, Serialize)]
struct EmitMetadata {
    pub source_hash: u64,
    pub target_hash: u64,
}

/// Cache for previously emitted files.
pub struct EmitCache {
    disk_cache: DiskCache,
    cli_version: &'static str,
}

impl EmitCache {
    /// Returns a new [`EmitCache`].
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            disk_cache: DiskCache::new(path),
            cli_version: "",
        }
    }

    pub fn read_emit(&self, specifier: &ModuleSpecifier, source_hash: u64) -> Option<String> {
        let meta_filename = self.get_meta_filename(specifier)?;
        let emit_filename = self.get_emit_filename(specifier)?;

        // Load and verify the metadata file is for this source and CLI version.
        let bytes = self.disk_cache.read(&meta_filename).ok()?;
        let meta: EmitMetadata = serde_json::from_slice(&bytes).ok()?;
        if meta.source_hash != source_hash {
            return None;
        }

        // Load and verify the compilation result is for the metadata.
        let emit_bytes = self.disk_cache.read(&emit_filename).ok()?;
        if meta.target_hash != compute_emit_hash(&emit_bytes, self.cli_version) {
            return None;
        }

        String::from_utf8(emit_bytes).ok()
    }

    pub fn write_emit(&self, specifier: &ModuleSpecifier, source_hash: u64, target_code: &str) {
        if let Err(err) = self.write_emit_inner(specifier, source_hash, target_code) {
            // Should never error here, but if it ever does don't fail.
            if cfg!(debug_assertions) {
                panic!("Error saving emit data ({specifier}): {err}");
            } else {
                // log::debug!("Error saving emit data({}): {}", specifier, err);
            }
        }
    }

    fn write_emit_inner(
        &self,
        specifier: &ModuleSpecifier,
        source_hash: u64,
        code: &str,
    ) -> Result<(), AnyError> {
        let meta_filename = self
            .get_meta_filename(specifier)
            .ok_or_else(|| anyhow!("Could not get meta filename."))?;
        let emit_filename = self
            .get_emit_filename(specifier)
            .ok_or_else(|| anyhow!("Could not get emit filename."))?;

        let target_hash = compute_emit_hash(code.as_bytes(), self.cli_version);
        let metadata = EmitMetadata {
            source_hash,
            target_hash,
        };

        let metadata = serde_json::to_vec(&metadata)?;
        self.disk_cache.write(&meta_filename, &metadata)?;
        self.disk_cache.write(&emit_filename, code.as_bytes())?;

        Ok(())
    }

    fn get_meta_filename(&self, specifier: &ModuleSpecifier) -> Option<PathBuf> {
        self.disk_cache
            .get_filename_with_extension(specifier, "meta")
    }

    fn get_emit_filename(&self, specifier: &ModuleSpecifier) -> Option<PathBuf> {
        self.disk_cache.get_filename_with_extension(specifier, "js")
    }
}

fn compute_emit_hash(bytes: &[u8], cli_version: &str) -> u64 {
    todo!()
}
