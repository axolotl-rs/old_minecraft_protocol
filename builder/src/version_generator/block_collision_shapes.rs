use crate::GenResult;
use minecraft_data_rs::models::block_collision_shapes::BlockCollisionShapes;
use std::fs::write;
use std::path::PathBuf;

use crate::error::GenError;

pub type Root = BlockCollisionShapes;
pub fn generate_block_collision_shapes(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
