use crate::GenResult;
use minecraft_data_rs::models::food::Food;
use std::fs::write;
use std::path::PathBuf;

use crate::error::GenError;

pub type Root = Vec<Food>;

pub fn generate_foods(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
