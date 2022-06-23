use crate::error::GenError;
use crate::GenResult;
use minecraft_data_rs::models::recipe::Recipe;
use std::collections::HashMap;
use std::fs::write;
use std::path::PathBuf;
pub type Root = HashMap<u32, Vec<Recipe>>;
pub fn generate_recipes(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
