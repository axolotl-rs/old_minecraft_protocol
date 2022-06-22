use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};
use minecraft_data_rs::models::protocol::types::TypeName;
use crate::code_gen::Field;
use crate::error::GenError;
use crate::version_generator::protocol::types::{GenerationResult, TypesGenerator};

pub fn generate_child_level_container(parent_name: String, built: Vec<(TypeName, Box<PacketDataType>)>, state: &TypesGenerator) -> Result<GenerationResult, GenError> {
    Ok(GenerationResult::Failure(PacketDataType::Built { name: parent_name.into(), value: NativeType::Container(built) }))

}
pub fn generate_top_level_container(name: TypeName, built: Vec<(TypeName, Box<PacketDataType>)>, state: &TypesGenerator) -> Result<GenerationResult, GenError> {

    let mut fields:Vec<Field> = Vec::with_capacity(built.len());
    //for (field_name, field_type) in built {

    //}
    Ok(GenerationResult::Failure(PacketDataType::Built { name, value: NativeType::Container(built) }))
}