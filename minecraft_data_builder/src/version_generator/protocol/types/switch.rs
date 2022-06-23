use crate::code_gen::{
    CompareTo, DataType, Field, GenerateType, InnerType, LanguageType, SwitchVariant,
    SwitchVariantType,
};
use crate::error::GenError;
use crate::version_generator::protocol::types::{
    GenerationResult, SubTypeResponse, TypesGenerator,
};
use convert_case::{Case, Casing};
use git2::SubmoduleUpdate::Default;
use log::warn;
use minecraft_data_rs::models::protocol::types::{SwitchType, TypeName};
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType};
use std::collections::HashMap;
pub fn generate_top_level_switch(
    name: TypeName,
    compare_to: String,
    default: Option<String>,
    fields: HashMap<String, SwitchType>,
    state: &TypesGenerator,
) -> Result<GenerationResult, GenError> {
    let mut variants = Vec::with_capacity(fields.len());
    let mut top_children = Vec::with_capacity(1);
    for (requirement, data) in fields.iter() {
        if let SwitchType::Type(data) = data {
            let result = state.sub_type(name.to_string(), data.clone(), |v| {
                warn!("A Switch was being put directly in a switch{}", v);
                CompareTo::Specified {
                    compare_to: v.to_string(),
                    compare_to_type: Box::new(DataType::default()),
                }
            })?;
            match result {
                SubTypeResponse::NotBuiltYet(built) => {
                    return Ok(GenerationResult::FailureMissingSubType(*built));
                }
                SubTypeResponse::CanNotBuild(build) => {
                    warn!("Can not build type: {}", requirement);
                    variants.push(SwitchVariant::new(
                        requirement.to_string(),
                        SwitchVariantType::Single(Field::new(
                            &TypeName::Named("value".to_string()),
                            DataType::default(),
                        )),
                    ));
                }
                SubTypeResponse::AlreadyBuilt(already_build) => {
                    variants.push(SwitchVariant::new(
                        requirement.to_string(),
                        SwitchVariantType::Single(Field::new(
                            &TypeName::Named("value".to_string()),
                            already_build.clone(),
                        )),
                    ));
                }
                SubTypeResponse::SubType {
                    data_type,
                    generate,
                } => match generate {
                    GenerateType::Container {
                        children,
                        fields,
                        content_name,
                    } => {
                        variants.push(SwitchVariant::new(
                            requirement.to_string(),
                            SwitchVariantType::Container(fields),
                        ));
                        children.into_iter().for_each(|v| top_children.push(v));
                    }
                    GenerateType::SwitchEnum { .. } => {
                        warn!("A Switch was being put directly in a switch{}", requirement);
                    }
                    GenerateType::Array { .. } => {
                        warn!("A array was being put directly in a switch{}", requirement);
                    }
                },
            };
        }
    }
    // For now assume all generic switches compare var_ints. We should change this in the future
    let to = CompareTo::Specified {
        compare_to,
        compare_to_type: Box::new(state.get_data_type("varint").unwrap().clone()),
    };
    let data_type = DataType::new_generated_type(
        name.to_string(),
        InnerType::Switch {
            compare_to: to.clone(),
        },
        LanguageType::Rust {
            absolute_path: format!(
                "super::{}::{}",
                name.to_string().to_case(Case::Snake),
                name.to_string().to_case(Case::UpperCamel)
            ),
        },
    );

    Ok(GenerationResult::Success {
        data_types: data_type,
        generate: GenerateType::SwitchEnum {
            content_name: name.to_string().to_case(Case::UpperCamel),
            compare_to: to,
            variants,
            children: top_children,
        },
    })
}

pub fn generate_child_level_switch(
    parent_name: String,
    compare_to: CompareTo,
    default: Option<String>,
    fields: HashMap<String, SwitchType>,
    state: &TypesGenerator,
) -> Result<GenerationResult, GenError> {
    let name = format!("{}Content", parent_name);

    let mut variants = Vec::with_capacity(fields.len());
    let mut top_children = Vec::with_capacity(1);
    for (requirement, data) in fields.iter() {
        if let SwitchType::Type(data) = data {
            let result = state.sub_type(name.to_string(), data.clone(), |v| {
                warn!("A Switch was being put directly in a switch{}", v);
                CompareTo::Specified {
                    compare_to: v.to_string(),
                    compare_to_type: Box::new(DataType::default()),
                }
            })?;
            match result {
                SubTypeResponse::NotBuiltYet(built) => {
                    return Ok(GenerationResult::FailureMissingSubType(*built));
                }
                SubTypeResponse::CanNotBuild(build) => {
                    warn!("Can not build type: {}", requirement);
                    variants.push(SwitchVariant::new(
                        requirement.to_string(),
                        SwitchVariantType::Single(Field::new(
                            &TypeName::Named("value".to_string()),
                            DataType::default(),
                        )),
                    ));
                }
                SubTypeResponse::AlreadyBuilt(already_build) => {
                    variants.push(SwitchVariant::new(
                        requirement.to_string(),
                        SwitchVariantType::Single(Field::new(
                            &TypeName::Named("value".to_string()),
                            already_build.clone(),
                        )),
                    ));
                }
                SubTypeResponse::SubType {
                    data_type,
                    generate,
                } => match generate {
                    GenerateType::Container {
                        children,
                        fields,
                        content_name,
                    } => {
                        variants.push(SwitchVariant::new(
                            requirement.to_string(),
                            SwitchVariantType::Container(fields),
                        ));
                        children.into_iter().for_each(|v| top_children.push(v));
                    }
                    GenerateType::SwitchEnum { .. } => {
                        warn!("A Switch was being put directly in a switch{}", requirement);
                    }
                    GenerateType::Array { .. } => {
                        warn!("A array was being put directly in a switch{}", requirement);
                    }
                },
            };
        }
    }
    // For now assume all generic switches compare var_ints. We should change this in the future
    let data_type = DataType::new_generated_type(
        name.to_string(),
        InnerType::Switch {
            compare_to: compare_to.clone(),
        },
        LanguageType::Rust {
            absolute_path: name.to_case(Case::UpperCamel),
        },
    );

    Ok(GenerationResult::Success {
        data_types: data_type,
        generate: GenerateType::SwitchEnum {
            content_name: name.to_string().to_case(Case::UpperCamel),
            compare_to: compare_to,
            variants,
            children: top_children,
        },
    })
}
