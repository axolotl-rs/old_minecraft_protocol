use crate::GenResult;
use minecraft_data_rs::models::item::Item;
use std::fs::write;
use std::path::PathBuf;

use crate::error::GenError;

pub type Root = Vec<Item>;

pub fn generate_items(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
