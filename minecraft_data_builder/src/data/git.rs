use crate::error::GenError;
use crate::{GenResult, Version};
use git2::Repository;
use std::fs::{read_dir, ReadDir};
use std::path::{Path, PathBuf};

pub struct GitFiles {
    versions_dir: PathBuf,
}

impl GitFiles {
    pub fn clone_repo<P: Into<PathBuf>>(repository_url: &str, path: P) -> GenResult<GitFiles> {
        let path = path.into();
        if !path.exists() {
            Repository::clone(repository_url, &path).map_err(GenError::Git2)?;
        }

        Ok(GitFiles {
            versions_dir: path.join("data"),
        })
    }
    pub fn get_data_path(&self) -> PathBuf {
        self.versions_dir.join("dataPaths.json")
    }
    pub fn join_directory(&self, path: &str) -> PathBuf {
        let mut buf = self.versions_dir.clone();
        path.split('/').for_each(|p| buf.push(p));
        buf
    }

    pub fn read_version_dir(&self, version: Version) -> GenResult<ReadDir> {
        read_dir(self.versions_dir.join(Path::new(version.name()))).map_err(GenError::Io)
    }
}
