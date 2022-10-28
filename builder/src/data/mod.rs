use log::debug;
use minecraft_data_rs::models::protocol::Protocol;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

use crate::data::git::GitFiles;
use crate::error::GenError;
use crate::version_generator::*;
use crate::{GenResult, Version};
use serde::Deserialize;

pub mod git;

pub struct MinecraftData {
    git_files: GitFiles,
}

#[derive(Deserialize)]
pub struct DataPathFile {
    pub pc: HashMap<String, FileLocation>,
}

#[derive(Deserialize)]
pub struct FileLocation {
    #[serde(flatten)]
    pub files: HashMap<String, String>,
}

impl MinecraftData {
    pub fn new(git_files: GitFiles) -> Self {
        Self { git_files }
    }

    pub fn fetch_version(&self, version: Version) -> GenResult<VersionData> {
        let version_dir = self.git_files.get_data_path();
        let path_lists: DataPathFile = serde_json::from_str(&read_to_string(version_dir)?)?;
        let option = path_lists.pc.get(version.name());
        if option.is_none() {
            return Err(GenError::Io(io::Error::new(
                io::ErrorKind::NotFound,
                "No such version",
            )));
        }
        let mut protocol = None;

        let mut version = None;

        for (name, path) in option.unwrap().files.iter() {
            let buf = self
                .git_files
                .join_directory(path)
                .join(format!("{}.json", name));
            debug!("Loading {}. File {:?}", name, buf);

            let string = read_to_string(buf)?;

            match name.as_str() {
                "protocol" => {
                    protocol = Some(
                        serde_json::from_str(&string)
                            .map_err(|err| GenError::Serde("protocol", err))?,
                    )
                }
                "version" => {
                    version = Some(
                        serde_json::from_str(&string)
                            .map_err(|err| GenError::Serde("version", err))?,
                    )
                }
                _ => continue,
            }
        }

        Ok(VersionData { protocol, version })
    }
}

pub struct VersionData {
    pub protocol: Option<Protocol>,

    pub version: Option<version::Root>,
}
