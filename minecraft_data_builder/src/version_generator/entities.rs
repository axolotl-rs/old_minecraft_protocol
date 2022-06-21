use std::fs::write;
use std::path::PathBuf;
use crate::GenResult;
use serde::Deserialize;
use serde::Serialize;
use crate::error::GenError;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub id: i64,
    pub internal_id: Option<i64>,
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub category: Option<String>,
}

pub fn generate_entities(file: PathBuf, _json: Root) -> GenResult<()> {
    let string = String::new();

    write(file, string).map_err(GenError::Io)
}