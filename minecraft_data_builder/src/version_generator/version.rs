use crate::GenResult;
use minecraft_data_rs::models::version::Version;
use std::fs::write;
use std::path::PathBuf;

use crate::error::GenError;

pub type Root = Version;

pub fn generate_version(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
