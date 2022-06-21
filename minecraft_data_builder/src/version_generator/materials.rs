use std::fs::write;
use std::path::PathBuf;
use crate::GenResult;
use serde::Serialize;
use serde::Deserialize;
use crate::error::GenError;

#[derive(Serialize, Deserialize)]
pub struct Root {
}

pub fn generate_materials(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}