use crate::error::GenError;
use crate::GenResult;
use serde::Deserialize;
use serde::Serialize;
use std::fs::write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Root {}

pub fn generate_materials(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
