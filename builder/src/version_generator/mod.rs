use crate::data::MinecraftData;
use crate::error::GenError;
use crate::GenResult;
use clap::ValueEnum;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub mod protocol;

pub mod version;

pub struct VersionGenerator {
    minecraft_data: MinecraftData,
    export_path: PathBuf,
}

impl VersionGenerator {
    pub fn new(minecraft_data: MinecraftData, export_path: PathBuf) -> Self {
        Self {
            minecraft_data,
            export_path,
        }
    }

    pub fn generate<'a>(&mut self, version: Version, _replace: bool) -> GenResult<()> {
        let version_data = self.minecraft_data.fetch_version(version)?;
        let version_dir = self.export_path.clone();

        create_dir_all(&version_dir).map_err(GenError::Io)?;

        if let Some(json) = version_data.protocol {
            protocol::generate_protocol(version_dir.join("protocol"), json, version)?;
        }

        if let Some(_json) = version_data.version {
            //  version::generate_version(version_dir.join("version.rs"), json)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum Version {
    One8,
    One19,
}

impl Version {
    pub fn name(self) -> &'static str {
        match self {
            Version::One8 => "1.8",
            Version::One19 => "1.19",
        }
    }

    pub fn safe_name(self) -> &'static str {
        match self {
            Version::One8 => "1_8",
            Version::One19 => "1_19",
        }
    }

    pub fn is_1_8(&self) -> bool {
        match self {
            Version::One8 => true,
            Version::One19 => false,
        }
    }
}
