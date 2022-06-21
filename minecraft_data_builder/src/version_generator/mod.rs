use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::data::MinecraftData;
use crate::error::GenError;
use crate::GenResult;

pub mod biomes;
pub mod block_collision_shapes;
pub mod blocks;
pub mod effects;
pub mod enchantments;
pub mod entities;
pub mod foods;
pub mod instruments;
pub mod items;
pub mod materials;
pub mod particles;
pub mod protocol;
pub mod recipes;
pub mod tints;
pub mod version;

pub struct VersionGenerator {
    minecraft_data: MinecraftData,
    export_path: PathBuf,
}

impl VersionGenerator {
    pub fn new(minecraft_data: MinecraftData, export_path: PathBuf) -> Self {
        Self { minecraft_data, export_path }
    }

    pub fn generate<'a>(&mut self, version: Version, _replace: bool) -> GenResult<()> {
        let version_data = self.minecraft_data.fetch_version(version)?;
        let version_dir = self.export_path.join(format!("v{}", version.safe_name()));

        create_dir_all(&version_dir).map_err(GenError::Io)?;

        if let Some(_json) = version_data.biomes {
           // biomes::generate_biomes(version, version_dir.join("biomes.rs"), json)?;
        }

        if let Some(_json) = version_data.block_collision_shapes {
           // block_collision_shapes::generate_block_collision_shapes(
           //     version_dir.join("block_collision_shapes.rs"), json)?;
        }

        if let Some(_json) = version_data.blocks {
           // blocks::generate_blocks(version_dir.join("blocks.rs"), json)?;
        }

        if let Some(_json) = version_data.effects {
           // effects::generate_effects(version_dir.join("effects.rs"), json)?;
        }

        if let Some(_json) = version_data.enchantments {
          //  enchantments::generate_enchantments(version_dir.join("enchantments.rs"),
         //                                       json)?;
        }

        if let Some(_json) = version_data.entities {// entities::generate_entities(
            //version_dir.join("entities.rs"), json)?;
        }

        if let Some(_json) = version_data.foods {// foods::generate_foods(
           // version_dir.join("foods.rs"), json)?;
        }

        if let Some(_json) = version_data.instruments {
          //  instruments::generate_instruments(version_dir.join("instruments.rs"), json)?;
        }

        if let Some(_json) = version_data.items {
          //  items::generate_items(version_dir.join("items.rs"), json)?;
        }

        if let Some(_json) = version_data.materials {
           // materials::generate_materials(version_dir.join("materials.rs"), json)?;
        }

        if let Some(_json) = version_data.particles {
          //  particles::generate_particles(version_dir.join("particles.rs"), json)?;
        }

        if let Some(json) = version_data.protocol {
            protocol::generate_protocol(version_dir.join("protocol"), json)?;
        }

        if let Some(_json) = version_data.recipes {
          //  recipes::generate_recipes(version_dir.join("recipes.rs"), json)?;
        }

        if let Some(_json) = version_data.tints {
            //tints::generate_tints(version_dir.join("tints.rs"), json)?;
        }

        if let Some(_json) = version_data.version {
          //  version::generate_version(version_dir.join("version.rs"), json)?;
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, clap::ArgEnum )]
pub enum Version {
    One8,
    One19
}

impl Version {
    pub fn name(self) -> &'static str {
        match self {
            Version::One8 => "1.8",
            Version::One19 => "1.19"
        }
    }

    pub fn safe_name(self) -> &'static str {
        match self {
            Version::One8 => "1_8",
            Version::One19 => "1_19"
        }
    }

    pub fn is_1_8(&self) -> bool {
        match self {
            Version::One8 => true,
            Version::One19 => false
        }
    }
}

fn capitalize_string(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}