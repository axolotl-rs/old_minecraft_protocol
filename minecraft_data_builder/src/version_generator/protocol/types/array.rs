use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};
use minecraft_data_rs::models::protocol::types::TypeName;
use crate::error::GenError;
use crate::version_generator::protocol::types::{GenerationResult, TypesGenerator};

pub fn generate_child_level_array(parent_name: String, count_type: NativeType, data_type: PacketDataType, state: &TypesGenerator) -> Result<GenerationResult, GenError> {
    Ok(GenerationResult::Failure(PacketDataType::Built { name: parent_name.into(), value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }))

}
pub fn generate_top_level_array(name: TypeName, count_type: NativeType, data_type: PacketDataType, state: &TypesGenerator) -> Result<GenerationResult, GenError> {
    Ok(GenerationResult::Failure(PacketDataType::Built { name: name, value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }))

}
