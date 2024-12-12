// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Result, Write};
#[cfg(target_os = "windows")]
use std::path::{Component, Prefix};
use std::path::{Path, PathBuf};

use deno_cache_dir::url_to_filename;
#[cfg(target_os = "windows")]
use deno_core::url::Host;
use deno_core::url::Url;
#[cfg(target_os = "windows")]
use serde::{Deserialize, Serialize};

/// On-disk storage for previously emitted files.
pub struct DiskCache {
    root_path: PathBuf,
}

impl DiskCache {
    /// Returns a new [`DiskCache`].
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_owned();
        assert!(path.is_absolute());
        Self { root_path: path }
    }

    /// Returns the reference to the root path.
    #[inline]
    pub fn root_path(&self) -> &Path {
        self.root_path.as_path()
    }

    fn get_filename(&self, url: &Url) -> Option<PathBuf> {
        let mut out = PathBuf::new();
        let scheme = url.scheme();
        out.push(scheme);

        match scheme {
            "wasm" => {
                let host = url.host_str()?;
                // Windows doesn't support ":" in filenames, so we
                // represent port using a special string.
                let host_port = url
                    .port()
                    .map(|port| format!("{host}_PORT{port}"))
                    .unwrap_or_else(|| host.to_string());
                out.push(host_port);
                out.extend(url.path_segments()?);
            }
            "http" | "https" | "data" | "blob" => return url_to_filename(url).ok(),
            "file" => {
                let path = url.to_file_path().ok()?;
                let mut components = path.components();

                // Windows doesn't support ":" in filenames, so we need to extract disk
                // prefix, e.g.: file:///C:/deno/js/unit_test_runner.ts should produce:
                // file\c\deno\js\unit_test_runner.ts
                #[cfg(target_os = "windows")]
                if let Some(Component::Prefix(prefix)) = components.next() {
                    match prefix.kind() {
                        Prefix::Disk(disk_byte) | Prefix::VerbatimDisk(disk_byte) => {
                            let disk = (disk_byte as char).to_string();
                            out.push(disk);
                        }
                        Prefix::UNC(server, share) | Prefix::VerbatimUNC(server, share) => {
                            out.push("UNC");
                            let host = Host::parse(server.to_str()?).ok()?;
                            let host = host.to_string().replace(':', "_");
                            out.push(host);
                            out.push(share);
                        }
                        _ => unreachable!(),
                    }
                }

                // Must be relative, so strip forward slash.
                let without_forward_slash = components.as_path().strip_prefix("/");
                out = out.join(without_forward_slash.unwrap_or(components.as_path()));
            }
            _ => return None,
        };

        Some(out)
    }

    pub fn get_filename_with_extension(&self, url: &Url, extension: &str) -> Option<PathBuf> {
        let base = self.get_filename(url)?;

        match base.extension() {
            None => Some(base.with_extension(extension)),
            Some(ext) => {
                let original_extension = OsStr::to_str(ext).unwrap_or("tmp");
                let final_extension = format!("{original_extension}.{extension}");
                Some(base.with_extension(final_extension))
            }
        }
    }

    /// Reads the entire contents of a file into a bytes vector.
    pub fn read(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
        let buf = std::fs::read(path)?;
        Ok(buf)
    }

    /// Writes an entire buffer into the temporary file and then rename the file.
    pub fn write(&self, path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> Result<()> {
        let path = path.as_ref().to_owned();
        std::fs::create_dir_all(&path)?;

        let temp_path = self.gen_temp_path(&path);
        let mut file = File::open(&temp_path)?;
        file.write_all(buf.as_ref())?;

        let file_path = self.root_path.join(path);
        std::fs::rename(&temp_path, &file_path)?;
        Ok(())
    }

    /// Returns the temporary file path.
    fn gen_temp_path(&self, path: &Path) -> PathBuf {
        let seq: String = (0..4)
            .map(|_| format!("{:02x}", rand::random::<u8>()))
            .collect();

        self.root_path
            .join(path)
            .with_file_name(seq)
            .with_extension("tmp")
    }
}
