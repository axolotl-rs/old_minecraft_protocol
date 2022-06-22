mod container;
mod switch;
mod array;

use std::collections::VecDeque;
use log::{info, warn};
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType, PacketDataTypes};
use crate::code_gen::{DataType, GenerateType};
use crate::configs::type_impls::get_default_type_impl;
use crate::error::GenError;

pub struct TypesGenerator {
    pub data_types: Vec<DataType>,
    pub types_to_be_generated: VecDeque<PacketDataType>,
    pub types_failed_to_generate: Vec<PacketDataType>,
}

pub enum GenerationResult {
    Success {
        data_types: DataType,
        generate: GenerateType,
    },
    FailureMissingSubType(PacketDataType),
    Failure(PacketDataType),
}

impl TypesGenerator {
    pub fn new(types: PacketDataTypes) -> Result<TypesGenerator, GenError> {
        let config_values = get_default_type_impl()?;
        Ok(Self {
            data_types: config_values.into_iter().map(|x| x.into()).collect(),
            types_to_be_generated: types.types.into_iter().collect(),
            types_failed_to_generate: Vec::new(),
        })
    }

    pub fn contains_data_type(&self, name: &str) -> bool {
        self.data_types.iter().any(|x| x.minecraft_name == name)
    }
    pub fn get_data_type(&self, name: &str) -> Option<&DataType> {
        self.data_types.iter().find(|x| x.minecraft_name == name)
    }
    pub fn generate(&mut self) -> Result<(), GenError> {
        while let Some(type_to_be_generated) = self.types_to_be_generated.pop_front() {
            match type_to_be_generated {
                PacketDataType::Native(key) => {
                    warn!("Top Level native type found, skipping {:?}", key);
                    self.types_failed_to_generate.push(PacketDataType::Native(key));
                }

                PacketDataType::UnknownNativeType(key) => {
                    warn!("Top Level UnknownNativeType type , skipping {:?}", key);
                    self.types_failed_to_generate.push(PacketDataType::UnknownNativeType(key));
                }
                PacketDataType::Built { name, value } => {
                    if self.contains_data_type(&name.to_string()) {
                        warn!("Top Level Built type already found, skipping {:?}", name);
                        self.types_failed_to_generate.push(PacketDataType::Built { name, value });
                        continue;
                    }
                    info!("Generating built type: {}", name);
                    let value = match value {
                        NativeType::Container(container) => {
                            container::generate_top_level_container(name, container, self)?
                        }
                        NativeType::Switch { compare_to, default, fields } => {
                            switch::generate_top_level_switch(name, compare_to, default, fields, self)?
                        }
                        NativeType::Array { count_type, array_type } => {
                            array::generate_top_level_array(name, *count_type, *array_type, self)?
                        }
                        v => {
                            warn!("Top Level Built type is not a supported type, skipping {:?}", name);
                            self.types_failed_to_generate.push(PacketDataType::Built { name, value: v });
                            continue;
                        }
                    };
                    match value {
                        GenerationResult::Success { generate, data_types } => {
                            self.data_types.push(data_types);
                            generate.generate_type();
                        }
                        GenerationResult::FailureMissingSubType(retu) => {
                            if self.types_to_be_generated.is_empty() {
                                warn!("Type could not be generated.");
                                self.types_failed_to_generate.push(retu);
                            } else {
                                warn!("Type could not be generated. Going to try again later");
                                self.types_to_be_generated.push_back(retu);
                            }

                            continue;
                        }
                        GenerationResult::Failure(retu) => {
                            warn!("Type could not be generated.");
                            self.types_failed_to_generate.push(retu);
                            continue;
                        }
                    }
                }
                PacketDataType::Other { name, value } => {
                    warn!("Top Level Other type found, skipping {:?}", name);
                    self.types_failed_to_generate.push(PacketDataType::Other { name, value });
                }
            }
        }
        Ok(())
    }
    pub fn sub_type(&self, parent_name: String, packet_data_type: Box<PacketDataType>) -> Result<SubTypeResponse, GenError> {
        match *packet_data_type {
            PacketDataType::Native(native) => {
                if let Some(data) = self.get_data_type(&native.to_string()) {
                    Ok(SubTypeResponse::AlreadyBuilt(data.clone()))
                } else {
                    match native {
                        NativeType::Container(fields) => {
                            let response = container::generate_child_level_container(parent_name, fields, self)?;
                            match response {
                                GenerationResult::Success { generate, data_types } => {
                                    Ok(SubTypeResponse::SubType {
                                        data_type: data_types,
                                        generate,
                                    })
                                }
                                GenerationResult::FailureMissingSubType(data) => {
                                    Ok(SubTypeResponse::NotBuiltYet(Box::new(data)))
                                }
                                GenerationResult::Failure(data) => {
                                    Ok(SubTypeResponse::CanNotBuild(Box::new(data)))
                                }
                            }
                        }
                        NativeType::Switch { compare_to, default, fields } => {
                            let response = switch::generate_child_level_switch(parent_name, compare_to, default, fields, self)?;
                            match response {
                                GenerationResult::Success { generate, data_types } => {
                                    Ok(SubTypeResponse::SubType {
                                        data_type: data_types,
                                        generate,
                                    })
                                }
                                GenerationResult::FailureMissingSubType(data) => {
                                    Ok(SubTypeResponse::NotBuiltYet(Box::new(data)))
                                }
                                GenerationResult::Failure(data) => {
                                    Ok(SubTypeResponse::CanNotBuild(Box::new(data)))
                                }
                            }
                        }
                        NativeType::Array { count_type, array_type } => {
                            let response = array::generate_child_level_array(parent_name, *count_type, *array_type, self)?;
                            match response {
                                GenerationResult::Success { generate, data_types } => {
                                    Ok(SubTypeResponse::SubType {
                                        data_type: data_types,
                                        generate,
                                    })
                                }
                                GenerationResult::FailureMissingSubType(data) => {
                                    Ok(SubTypeResponse::NotBuiltYet(Box::new(data)))
                                }
                                GenerationResult::Failure(data) => {
                                    Ok(SubTypeResponse::CanNotBuild(Box::new(data)))
                                }
                            }
                        }
                        _ => {
                            Ok(SubTypeResponse::CanNotBuild(Box::from(PacketDataType::Native(native))))
                        }
                    }
                }
            }
            PacketDataType::Built { name, value } => {
                warn!("Trying to build a built type as a sub type");
                self.sub_type(name.to_string(), Box::new(PacketDataType::Native(value)))
            }
            PacketDataType::Other { name, value } => {
                if let Some(inner_name) = name.as_ref() {
                    if let Some(data) = self.get_data_type(&inner_name.to_string()) {
                        Ok(SubTypeResponse::AlreadyBuilt(data.clone()))
                    } else {
                        Ok(SubTypeResponse::NotBuiltYet(Box::new(PacketDataType::Other { name, value })))
                    }
                } else {
                    Ok(SubTypeResponse::NotBuiltYet(Box::new(PacketDataType::Other { name, value })))
                }
            }
            data => {
                Ok(SubTypeResponse::NotBuiltYet(Box::new(data)))
            }
        }
    }
}


pub enum SubTypeResponse {
    NotBuiltYet(Box<PacketDataType>),
    CanNotBuild(Box<PacketDataType>),
    AlreadyBuilt(DataType),
    SubType {
        data_type: DataType,
        generate: GenerateType,
    },
}
