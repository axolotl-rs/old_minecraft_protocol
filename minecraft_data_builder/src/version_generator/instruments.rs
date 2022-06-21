use std::fs::write;
use std::path::PathBuf;
use crate::GenResult;
use serde::Serialize;
use serde::Deserialize;
use crate::error::GenError;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub name: String,
}

pub fn generate_instruments(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}