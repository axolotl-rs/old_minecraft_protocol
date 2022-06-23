use crate::code_gen::{DataType, InnerType, LanguageType};
use crate::error::GenError;
pub use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PacketContentLocation {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: PacketContentType,
    pub switch: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum PacketContentType {
    Native {
        #[serde(rename = "type")]
        rust_type: String,
    },
    PreDefined {
        path: String,
    },
}

impl From<PacketContentType> for LanguageType {
    fn from(value: PacketContentType) -> Self {
        match value {
            PacketContentType::Native { rust_type } => LanguageType::Rust {
                absolute_path: rust_type,
            },
            PacketContentType::PreDefined { path } => LanguageType::Rust {
                absolute_path: path,
            },
        }
    }
}

impl From<PacketContentLocation> for DataType {
    fn from(packet: PacketContentLocation) -> Self {
        DataType {
            minecraft_name: packet.name,
            inner_type: InnerType::Container,
            language_type: packet.typ.into(),
        }
    }
}

pub fn get_default_type_impl() -> Result<Vec<PacketContentLocation>, GenError> {
    serde_json::from_slice(
        crate::Configs::get("default_type_impls.json")
            .unwrap()
            .data
            .as_ref(),
    )
    .map_err(|e| GenError::from(e))
}
