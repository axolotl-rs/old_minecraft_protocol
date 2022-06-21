use std::collections::HashMap;
use std::fs::{read_to_string};
use std::io;
use log::debug;
use minecraft_data_rs::models::protocol::Protocol;

use crate::error::GenError;
use crate::{GenResult, Version};
use crate::data::git::GitFiles;
use crate::version_generator::*;
use serde::Deserialize;
pub mod git;

pub struct MinecraftData {
    git_files: GitFiles
}
#[derive(Deserialize)]
pub struct DataPathFile{
    pub pc: HashMap<String, FileLocation>
}
#[derive(Deserialize)]
pub struct FileLocation{
    #[serde(flatten)]
    pub files: HashMap<String, String>
}
impl MinecraftData {
    pub fn new(git_files: GitFiles) -> Self {
        Self { git_files }
    }

    pub fn fetch_version(&self, version: Version) -> GenResult<VersionData> {
        let version_dir = self.git_files.get_data_path();
        let path_lists :DataPathFile = serde_json::from_str(&read_to_string(version_dir)?)?;
        let option = path_lists.pc.get(version.name());
        if option.is_none(){
            return Err(GenError::Io(io::Error::new(io::ErrorKind::NotFound, "No such version")));
        }
        let mut biomes = None;
        let mut block_collision_shapes = None;
        let mut blocks = None;
        let mut effects = None;
        let mut enchantments = None;
        let mut entities = None;
        let mut foods = None;
        let mut instruments = None;
        let mut items = None;
        let mut materials = None;
        let mut particles = None;
        let mut protocol = None;
        let mut recipes = None;
        let mut tints = None;
        let mut version = None;

        for (name, path) in option.unwrap().files.iter() {
            let buf = self.git_files.join_directory(path).join(format!("{}.json", name));
            debug!("Loading {}. File {:?}", name, buf);

            let string = read_to_string(buf)?;

            match name.as_str() {
                "biomes" => biomes = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("biomes", err))?
                ),
                "block_collision_shapes" => block_collision_shapes = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("block_collision_shapes", err))?
                ),
                "blocks" => blocks = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("blocks", err))?
                ),
                "effects" => effects = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("effects", err))?
                ),
                "enchantments" => enchantments = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("enchantments", err))?
                ),
                "entities" => entities = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("entities", err))?
                ),
                "foods" => foods = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("foods", err))?
                ),
                "instruments" => instruments = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("instruments", err))?
                ),
                "items" => items = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("items", err))?
                ),
                "materials" => materials = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("materials", err))?
                ),
                "particles" => particles = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("particles", err))?
                ),
                "protocol" => protocol = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("protocol", err))?
                ),
                "recipes" => recipes = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("recipes", err))?
                ),
                "tints" => tints = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("tints", err))?
                ),
                "version" => version = Some(
                    serde_json::from_str(&string).map_err(|err| GenError::Serde("version", err))?
                ),
                _ => continue,
            }
        }

        Ok(VersionData {
            biomes, block_collision_shapes,
            blocks, effects, enchantments,
            entities, foods,
            instruments, items, materials, particles,
            protocol, recipes, tints, version
        })
    }
}

pub struct VersionData {
    pub biomes: Option<biomes::Root>,
    pub block_collision_shapes: Option<block_collision_shapes::Root>,
    pub blocks: Option<blocks::Root>,
    pub effects: Option<effects::Root>,
    pub enchantments: Option<enchantments::Root>,
    pub entities: Option<entities::Root>,
    pub foods: Option<foods::Root>,
    pub instruments: Option<instruments::Root>,
    pub items: Option<items::Root>,
    pub materials: Option<materials::Root>,
    pub particles: Option<particles::Root>,
    pub protocol: Option<Protocol>,
    pub recipes: Option<recipes::Root>,
    pub tints: Option<tints::Root>,
    pub version: Option<version::Root>,
}