// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.
// Copyright 2023-2024 the Supabase authors. All rights reserved. MIT license.

use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use deno_fs::{AccessCheckCb, FileSystem, FsDirEntry, FsFileType, OpenOptions};
use deno_io::fs::{File, FsResult, FsStat};

use crate::runtime::filesystem::FileBackedVfs;

#[derive(Debug, Clone)]
pub struct StaticFs {
    inner: Arc<FileBackedVfs>,
}

#[async_trait::async_trait(?Send)]
impl FileSystem for StaticFs {
    fn cwd(&self) -> FsResult<PathBuf> {
        todo!()
    }

    fn tmp_dir(&self) -> FsResult<PathBuf> {
        todo!()
    }

    fn chdir(&self, path: &Path) -> FsResult<()> {
        todo!()
    }

    fn umask(&self, mask: Option<u32>) -> FsResult<u32> {
        todo!()
    }

    fn open_sync(
        &self,
        path: &Path,
        options: OpenOptions,
        access_check: Option<AccessCheckCb>,
    ) -> FsResult<Rc<dyn File>> {
        todo!()
    }

    async fn open_async<'a>(
        &'a self,
        path: PathBuf,
        options: OpenOptions,
        access_check: Option<AccessCheckCb<'a>>,
    ) -> FsResult<Rc<dyn File>> {
        todo!()
    }

    fn mkdir_sync(&self, path: &Path, recursive: bool, mode: u32) -> FsResult<()> {
        todo!()
    }

    async fn mkdir_async(&self, path: PathBuf, recursive: bool, mode: u32) -> FsResult<()> {
        todo!()
    }

    fn chmod_sync(&self, path: &Path, mode: u32) -> FsResult<()> {
        todo!()
    }

    async fn chmod_async(&self, path: PathBuf, mode: u32) -> FsResult<()> {
        todo!()
    }

    fn chown_sync(&self, path: &Path, uid: Option<u32>, gid: Option<u32>) -> FsResult<()> {
        todo!()
    }

    async fn chown_async(&self, path: PathBuf, uid: Option<u32>, gid: Option<u32>) -> FsResult<()> {
        todo!()
    }

    fn lchown_sync(&self, path: &Path, uid: Option<u32>, gid: Option<u32>) -> FsResult<()> {
        todo!()
    }

    async fn lchown_async(
        &self,
        path: PathBuf,
        uid: Option<u32>,
        gid: Option<u32>,
    ) -> FsResult<()> {
        todo!()
    }

    fn remove_sync(&self, path: &Path, recursive: bool) -> FsResult<()> {
        todo!()
    }

    async fn remove_async(&self, path: PathBuf, recursive: bool) -> FsResult<()> {
        todo!()
    }

    fn copy_file_sync(&self, oldpath: &Path, newpath: &Path) -> FsResult<()> {
        todo!()
    }

    async fn copy_file_async(&self, oldpath: PathBuf, newpath: PathBuf) -> FsResult<()> {
        todo!()
    }

    fn cp_sync(&self, path: &Path, new_path: &Path) -> FsResult<()> {
        todo!()
    }

    async fn cp_async(&self, path: PathBuf, new_path: PathBuf) -> FsResult<()> {
        todo!()
    }

    fn stat_sync(&self, path: &Path) -> FsResult<FsStat> {
        todo!()
    }

    async fn stat_async(&self, path: PathBuf) -> FsResult<FsStat> {
        todo!()
    }

    fn lstat_sync(&self, path: &Path) -> FsResult<FsStat> {
        todo!()
    }

    async fn lstat_async(&self, path: PathBuf) -> FsResult<FsStat> {
        todo!()
    }

    fn realpath_sync(&self, path: &Path) -> FsResult<PathBuf> {
        todo!()
    }

    async fn realpath_async(&self, path: PathBuf) -> FsResult<PathBuf> {
        todo!()
    }

    fn read_dir_sync(&self, path: &Path) -> FsResult<Vec<FsDirEntry>> {
        todo!()
    }

    async fn read_dir_async(&self, path: PathBuf) -> FsResult<Vec<FsDirEntry>> {
        todo!()
    }

    fn rename_sync(&self, oldpath: &Path, newpath: &Path) -> FsResult<()> {
        todo!()
    }

    async fn rename_async(&self, oldpath: PathBuf, newpath: PathBuf) -> FsResult<()> {
        todo!()
    }

    fn link_sync(&self, oldpath: &Path, newpath: &Path) -> FsResult<()> {
        todo!()
    }

    async fn link_async(&self, oldpath: PathBuf, newpath: PathBuf) -> FsResult<()> {
        todo!()
    }

    fn symlink_sync(
        &self,
        oldpath: &Path,
        newpath: &Path,
        file_type: Option<FsFileType>,
    ) -> FsResult<()> {
        todo!()
    }

    async fn symlink_async(
        &self,
        oldpath: PathBuf,
        newpath: PathBuf,
        file_type: Option<FsFileType>,
    ) -> FsResult<()> {
        todo!()
    }

    fn read_link_sync(&self, path: &Path) -> FsResult<PathBuf> {
        todo!()
    }

    async fn read_link_async(&self, path: PathBuf) -> FsResult<PathBuf> {
        todo!()
    }

    fn truncate_sync(&self, path: &Path, len: u64) -> FsResult<()> {
        todo!()
    }

    async fn truncate_async(&self, path: PathBuf, len: u64) -> FsResult<()> {
        todo!()
    }

    fn utime_sync(
        &self,
        path: &Path,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        todo!()
    }

    async fn utime_async(
        &self,
        path: PathBuf,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        todo!()
    }

    fn lutime_sync(
        &self,
        path: &Path,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        todo!()
    }

    async fn lutime_async(
        &self,
        path: PathBuf,
        atime_secs: i64,
        atime_nanos: u32,
        mtime_secs: i64,
        mtime_nanos: u32,
    ) -> FsResult<()> {
        todo!()
    }
}
