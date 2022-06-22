use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};
use minecraft_data_rs::models::protocol::types::TypeName;
use crate::error::GenError;
use crate::version_generator::protocol::types::{GenerationResult, TypesGenerator};

pub fn generate_top_level_container(name:  TypeName, built: Vec<(TypeName, Box<PacketDataType>)>, state: &TypesGenerator) ->Result<GenerationResult, GenError>{

}