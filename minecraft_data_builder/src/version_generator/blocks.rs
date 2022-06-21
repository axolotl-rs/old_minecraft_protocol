use std::fs::write;
use std::path::PathBuf;
use minecraft_data_rs::models::block::Block;
use crate::GenResult;
use crate::error::GenError;




pub type Root = Vec<Block>;



pub fn generate_blocks(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}