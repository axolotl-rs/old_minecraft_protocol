use crate::error::GenError;
use crate::GenResult;
use serde::Deserialize;
use serde::Serialize;
use std::fs::write;
use std::path::PathBuf;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub fn generate_effects(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}
