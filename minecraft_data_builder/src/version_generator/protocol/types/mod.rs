mod container;
mod switch;

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
    Success{
        data_types: DataType,
        generate: GenerateType
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
    pub fn generate(&mut self) -> Result<(), GenError> {
        while let Some(type_to_be_generated) = self.types_to_be_generated.pop_front() {
            match type_to_be_generated {
                PacketDataType::Native(key) => {
                    warn!("Top Level native type found, skipping {:?}", key);
                    self.types_failed_to_generate.push(PacketDataType::Native(key));
                }

                PacketDataType::UnknownNativeType(v) => {
                    warn!("Top Level UnknownNativeType type , skipping {:?}", key);
                    self.types_failed_to_generate.push(PacketDataType::UnknownNativeType(v));
                }
                PacketDataType::Built { name, value } => {
                    if self.contains_data_type(&name.to_string()) {
                        warn!("Top Level Built type already found, skipping {:?}", key);
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
                        v => {
                            warn!("Top Level Built type is not a supported type, skipping {:?}", key);
                            self.types_failed_to_generate.push(PacketDataType::Built { name, value: v });
                            continue;
                        }
                    };
                    match value {
                        GenerationResult::Success { generate,data_types } => {
                            self.data_types.push(data_types);
                            generate.generate_type();
                        }
                        GenerationResult::FailureMissingSubType(retu) => {
                            if self.types_to_be_generated.is_empty(){
                                warn!("Type could not be generated.");
                                self.types_failed_to_generate.push(retu);
                            }else{
                                warn!("Type could not be generated. Going to try again later");
                                self.types_to_be_generated.push(retu);
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
                PacketDataType::Other(key, v) => {
                    warn!("Top Level Other type found, skipping {:?}", key);
                    self.types_failed_to_generate.push(PacketDataType::Other(key, v));
                }
            }
        }
        Ok(())
    }
}
