// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::rc::Rc;

use deno_core::{BufMutView, BufView, ResourceHandleFd, WriteOutcome};
use deno_io::fs::{File, FsError, FsResult, FsStat};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FileBackedVfsRoot {
    pub directory: VirtualDirectory,
    pub root_path: PathBuf,
}

#[derive(Debug)]
pub struct FileBackedVfs {
    fs_root: FileBackedVfsRoot,
}

impl FileBackedVfs {
    /// Returns a new [`FileBackedVfs`].
    pub fn new() -> Self {
        todo!()
    }

    #[inline]
    pub fn is_path_within(&self, path: &Path) -> bool {
        path.starts_with(&self.fs_root.root_path)
    }
}

#[derive(Debug, Clone)]
pub struct FileBackedVfsFile {}

impl FileBackedVfsFile {}

#[async_trait::async_trait(?Send)]
impl File for FileBackedVfsFile {
    fn read_sync(self: Rc<Self>, buf: &mut [u8]) -> FsResult<usize> {
        Err(FsError::NotSupported)
    }

    async fn read_byob(self: Rc<Self>, buf: BufMutView) -> FsResult<(usize, BufMutView)> {
        Err(FsError::NotSupported)
    }

    fn write_sync(self: Rc<Self>, buf: &[u8]) -> FsResult<usize> {
        Err(FsError::NotSupported)
    }

    async fn write(self: Rc<Self>, buf: BufView) -> FsResult<WriteOutcome> {
        Err(FsError::NotSupported)
    }

    fn write_all_sync(self: Rc<Self>, buf: &[u8]) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn write_all(self: Rc<Self>, buf: BufView) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn read_all_sync(self: Rc<Self>) -> FsResult<Vec<u8>> {
        Err(FsError::NotSupported)
    }

    async fn read_all_async(self: Rc<Self>) -> FsResult<Vec<u8>> {
        Err(FsError::NotSupported)
    }

    fn chmod_sync(self: Rc<Self>, path_mode: u32) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn chmod_async(self: Rc<Self>, mode: u32) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn seek_sync(self: Rc<Self>, pos: SeekFrom) -> FsResult<u64> {
        Err(FsError::NotSupported)
    }

    async fn seek_async(self: Rc<Self>, pos: SeekFrom) -> FsResult<u64> {
        Err(FsError::NotSupported)
    }

    fn datasync_sync(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn datasync_async(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn sync_sync(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn sync_async(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn stat_sync(self: Rc<Self>) -> FsResult<FsStat> {
        Err(FsError::NotSupported)
    }

    async fn stat_async(self: Rc<Self>) -> FsResult<FsStat> {
        Err(FsError::NotSupported)
    }

    fn lock_sync(self: Rc<Self>, exclusive: bool) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn lock_async(self: Rc<Self>, exclusive: bool) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn unlock_sync(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn unlock_async(self: Rc<Self>) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn truncate_sync(self: Rc<Self>, len: u64) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn truncate_async(self: Rc<Self>, len: u64) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    fn utime_sync(
        self: Rc<Self>,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    async fn utime_async(
        self: Rc<Self>,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        Err(FsError::NotSupported)
    }

    #[inline]
    fn as_stdio(self: Rc<Self>) -> FsResult<Stdio> {
        Err(FsError::NotSupported)
    }

    #[inline]
    fn backing_fd(self: Rc<Self>) -> Option<ResourceHandleFd> {
        None
    }

    #[inline]
    fn try_clone_inner(self: Rc<Self>) -> FsResult<Rc<dyn File>> {
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VfsEntry {
    Directory(VirtualDirectory),
    File(VirtualFile),
    Symlink(VirtualSymlink),
}

#[derive(Debug)]
enum VfsEntryRef<'a> {
    Directory(&'a VirtualDirectory),
    File(&'a VirtualFile),
    Symlink(&'a VirtualSymlink),
}

impl<'a> VfsEntryRef<'a> {
    /// Returns a new [`FsStat`].
    pub fn as_fs_stat(&self) -> FsStat {
        match self {
            VfsEntryRef::Directory(x) => x.as_fs_stat(),
            VfsEntryRef::File(x) => x.as_fs_stat(),
            VfsEntryRef::Symlink(x) => x.as_fs_stat(),
        }
    }
}

impl VfsEntry {
    /// Returns a reference to the entries name.
    pub fn name(&self) -> &str {
        match self {
            VfsEntry::Directory(dir) => &dir.name,
            VfsEntry::File(file) => &file.name,
            VfsEntry::Symlink(sm) => &sm.name,
        }
    }

    /// Returns a new [`VfsEntryRef`] from this entry.
    pub fn as_entry_ref(&self) -> VfsEntryRef<'_> {
        match self {
            VfsEntry::Directory(dir) => VfsEntryRef::Directory(dir),
            VfsEntry::File(file) => VfsEntryRef::File(file),
            VfsEntry::Symlink(sm) => VfsEntryRef::Symlink(sm),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualDirectory {
    pub name: String,
    // Should be sorted by name.
    pub entries: Vec<VfsEntry>,
}

impl VirtualDirectory {
    /// Returns a new [`FsStat`].
    pub fn as_fs_stat(&self) -> FsStat {
        FsStat {
            is_file: false,
            is_directory: true,
            is_symlink: false,
            size: 0,
            mtime: None,
            atime: None,
            birthtime: None,
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 0,
            blocks: 0,
            is_block_device: false,
            is_char_device: false,
            is_fifo: false,
            is_socket: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualFile {
    pub name: String,
    pub offset: u64,
    pub len: u64,
}

impl VirtualFile {
    /// Returns a new [`FsStat`].
    pub fn as_fs_stat(&self) -> FsStat {
        FsStat {
            is_file: true,
            is_directory: false,
            is_symlink: false,
            size: 0,
            mtime: None,
            atime: None,
            birthtime: None,
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 0,
            blocks: 0,
            is_block_device: false,
            is_char_device: false,
            is_fifo: false,
            is_socket: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualSymlink {
    pub name: String,
    pub dest_parts: Vec<String>,
}

impl VirtualSymlink {
    /// Returns a new [`FsStat`].
    pub fn as_fs_stat(&self) -> FsStat {
        FsStat {
            is_file: false,
            is_directory: false,
            is_symlink: true,
            size: 0,
            mtime: None,
            atime: None,
            birthtime: None,
            dev: 0,
            ino: 0,
            mode: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize: 0,
            blocks: 0,
            is_block_device: false,
            is_char_device: false,
            is_fifo: false,
            is_socket: false,
        }
    }

    pub fn resolve_dest_from_root(&self, root: &Path) -> PathBuf {
        let mut dest = root.to_path_buf();
        for part in &self.dest_parts {
            dest.push(part);
        }
        dest
    }
}
