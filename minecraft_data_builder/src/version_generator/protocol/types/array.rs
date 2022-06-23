use convert_case::{Case, Casing};
use log::warn;
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};
use minecraft_data_rs::models::protocol::types::TypeName;
use crate::code_gen::{CompareTo, DataType, GenerateType, InnerType};
use crate::error::GenError;
use crate::version_generator::protocol::types::{GenerationResult, SubTypeResponse, TypesGenerator};

pub fn generate_child_level_array(parent_name: String, count_type: NativeType, data_type: PacketDataType, state: &TypesGenerator) -> Result<GenerationResult, GenError> {
    let mut children = Vec::with_capacity(1);
    let count = state.get_data_type(&count_type.to_string());
    let name = format!("{}Array", parent_name);

    if count.is_none() {
        return Ok(GenerationResult::Failure(PacketDataType::Built { name: name.into(), value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }));
    }
    let response = state.sub_type(name.to_string(), Box::new(data_type.clone()), |v| {
        warn!("A switch inside an array is not supported");
        CompareTo::Specified {
            compare_to: v.to_string(),
            compare_to_type: Box::new(DataType::default()),
        }
    })?;
    let data_type = match response {
        SubTypeResponse::NotBuiltYet(_) => {
            return Ok(GenerationResult::FailureMissingSubType(PacketDataType::Built { name: TypeName::Anonymous, value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }));
        }
        SubTypeResponse::CanNotBuild(build) => {
            warn!("Can not build type: {}", name);
            DataType::default()
        }
        SubTypeResponse::AlreadyBuilt(already_build) => {
            already_build
        }
        SubTypeResponse::SubType { data_type, generate } => {
            children.push(generate);
            data_type
        }
    };
    let new_type = DataType::new_generated_type(name.to_string(), InnerType::Array);
    Ok(
        GenerationResult::Success {
            data_types: new_type,
            generate: GenerateType::Array {
                content_name: name.to_string().to_case(Case::UpperCamel),
                count_type: count.unwrap().clone(),
                data_type: data_type,
                children,
            },
        }
    )}

pub fn generate_top_level_array(name: TypeName, count_type: NativeType, data_type: PacketDataType, state: &TypesGenerator) -> Result<GenerationResult, GenError> {
    let mut children = Vec::with_capacity(1);
    let count = state.get_data_type(&count_type.to_string());
    if count.is_none() {
        return Ok(GenerationResult::Failure(PacketDataType::Built { name: name.into(), value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }));
    }
    let response = state.sub_type(name.to_string(), Box::new(data_type.clone()), |v| {
        warn!("A switch inside an array is not supported");
        CompareTo::Specified {
            compare_to: v.to_string(),
            compare_to_type: Box::new(DataType::default()),
        }
    })?;
    let data_type = match response {
        SubTypeResponse::NotBuiltYet(_) => {
            return Ok(GenerationResult::FailureMissingSubType(PacketDataType::Built { name: TypeName::Anonymous, value: NativeType::Array { count_type: Box::new(count_type), array_type: Box::new((data_type)) } }));
        }
        SubTypeResponse::CanNotBuild(build) => {
            warn!("Can not build type: {}", name);
            DataType::default()
        }
        SubTypeResponse::AlreadyBuilt(already_build) => {
            already_build
        }
        SubTypeResponse::SubType { data_type, generate } => {
            children.push(generate);
            data_type
        }
    };
    let new_type = DataType::new_generated_type(name.to_string(), InnerType::Array);
    Ok(
        GenerationResult::Success {
            data_types: new_type,
            generate: GenerateType::Array {
                content_name: name.to_string().to_case(Case::UpperCamel),
                count_type: count.unwrap().clone(),
                data_type: data_type,
                children,
            },
        }
    )
}
