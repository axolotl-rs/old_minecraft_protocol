use crate::code_gen::{CompareTo, DataType, Field, GenerateType, InnerType, LanguageType};
use crate::error::GenError;
use crate::version_generator::protocol::types::{
    GenerationResult, SubTypeResponse, TypesGenerator,
};
use convert_case::{Case, Casing};
use log::warn;
use minecraft_data_rs::models::protocol::types::TypeName;
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};

pub fn generate_child_level_container(
    parent_name: String,
    built: Vec<(TypeName, Box<PacketDataType>)>,
    state: &TypesGenerator,
) -> Result<GenerationResult, GenError> {
    let mut fields: Vec<Field> = Vec::with_capacity(built.len());
    let mut children = Vec::with_capacity(1);
    let name = format!("{}Content", parent_name);

    for (field_name, field_type) in &built {
        let result = state.sub_type(name.to_string(), field_type.clone(), |v| {
            if let Some(v) = fields.iter().find(|f| f.name == v) {
                CompareTo::Specified {
                    compare_to: v.name.to_string(),
                    compare_to_type: Box::new(v.data_type.clone()),
                }
            } else {
                CompareTo::Specified {
                    compare_to: "not_found".to_string(),
                    compare_to_type: Box::new(Default::default()),
                }
            }
        })?;
        let data_type = match result {
            SubTypeResponse::NotBuiltYet(_) => {
                return Ok(GenerationResult::FailureMissingSubType(
                    PacketDataType::Built {
                        name: TypeName::Anonymous,
                        value: NativeType::Container(built),
                    },
                ));
            }
            SubTypeResponse::CanNotBuild(_build) => {
                warn!("Can not build type: {}", field_name);
                DataType::default()
            }
            SubTypeResponse::AlreadyBuilt(already_build) => already_build,
            SubTypeResponse::SubType {
                data_type,
                generate,
            } => {
                children.push(generate);
                data_type
            }
        };
        fields.push(Field::new(&field_name, data_type));
    }
    let data_type = DataType::new_generated_type(
        name.clone(),
        InnerType::Container,
        LanguageType::Rust {
            absolute_path: name.to_case(Case::UpperCamel),
        },
    );
    Ok(GenerationResult::Success {
        data_types: data_type,
        generate: GenerateType::Container {
            content_name: name.to_string().to_case(Case::UpperCamel),
            fields,
            children,
        },
    })
}

pub fn generate_top_level_container(
    name: TypeName,
    built: Vec<(TypeName, Box<PacketDataType>)>,
    state: &TypesGenerator,
) -> Result<GenerationResult, GenError> {
    let mut fields: Vec<Field> = Vec::with_capacity(built.len());
    let mut children = Vec::with_capacity(1);
    for (field_name, field_type) in &built {
        let result = state.sub_type(name.to_string(), field_type.clone(), |v| {
            if let Some(v) = fields.iter().find(|f| f.name == v) {
                CompareTo::Specified {
                    compare_to: v.name.to_string().to_case(Case::Snake),
                    compare_to_type: Box::new(v.data_type.clone()),
                }
            } else {
                CompareTo::Specified {
                    compare_to: "not_found".to_string(),
                    compare_to_type: Box::new(Default::default()),
                }
            }
        })?;
        let data_type = match result {
            SubTypeResponse::NotBuiltYet(_) => {
                return Ok(GenerationResult::FailureMissingSubType(
                    PacketDataType::Built {
                        name,
                        value: NativeType::Container(built),
                    },
                ));
            }
            SubTypeResponse::CanNotBuild(_build) => {
                warn!("Can not build type: {}", field_name);
                DataType::default()
            }
            SubTypeResponse::AlreadyBuilt(already_build) => already_build,
            SubTypeResponse::SubType {
                data_type,
                generate,
            } => {
                children.push(generate);
                data_type
            }
        };
        fields.push(Field::new(&field_name, data_type));
    }
    let data_type = DataType::new_generated_type(
        name.to_string(),
        InnerType::Container,
        LanguageType::Rust {
            absolute_path: format!(
                "{}::{}",
                name.to_string().to_case(Case::Snake),
                name.to_string().to_case(Case::UpperCamel)
            ),
        },
    );
    Ok(GenerationResult::Success {
        data_types: data_type,
        generate: GenerateType::Container {
            content_name: name.to_string().to_case(Case::UpperCamel),
            fields,
            children,
        },
    })
}
