pub mod attributes;
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
pub mod windows;
pub mod data;

pub trait Variation {
    type Metadata;
    type DisplayName;

    fn metadata(&self) -> Self::Metadata;

    fn display_name(&self) -> Self::DisplayName;
}