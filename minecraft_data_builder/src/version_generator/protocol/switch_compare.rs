use std::collections::HashMap;
use convert_case::{Case, Casing};


use log::{debug, warn};
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType, PacketDataTypes};
use minecraft_data_rs::models::protocol::types::{SwitchType, TypeName};
use crate::error::GenError;
use crate::handlesbars::{Field, GenerateType, SwitchVariant};
use crate::version_generator::protocol::{find_type, generate_packet_content, PacketContentLocation, prepare_field, prepare_key, switch_compare};
#[derive(Debug, Clone)]
pub enum CompareTo {
    Specified {
        compare_to: String,
        compare_to_type: String,
    },
    Generic,
}


pub fn create_switch(parent_name: &TypeName, wrap_in_module: bool, types: &mut Vec<PacketContentLocation>, fields: &HashMap<String, SwitchType>,
                     _default: &Option<String>,
                     compare_to: CompareTo, types_to_be_generated: &PacketDataTypes) -> Result<Vec<GenerateType>, GenError> {
    let mut variants = Vec::new();
    let mut children = Vec::new();
    let mut generates = Vec::with_capacity(1);
    for (key, value) in fields.iter() {
        if let SwitchType::Type(value) = value {
            debug!("Adding Switch Variant: {}::{} with data {:?}", parent_name, key, value);
            if let Some(typ) = find_type(value.as_ref(), types) {
                if typ.name.eq("void") {
                    let requirement = if let CompareTo::Specified { compare_to_type, .. } = &compare_to {
                        if compare_to_type.eq("string") {
                            let string = format!("\"{}\"", key);
                            string
                        } else {
                            key.to_string()
                        }
                    } else {
                        key.to_string()
                    };
                    variants.push(SwitchVariant { name: prepare_key(key.as_str()), requirement, void: true, fields: None, single: None })
                } else {
                    let requirement = if let CompareTo::Specified { compare_to_type, .. } = &compare_to {
                        if compare_to_type.eq_ignore_ascii_case("string") {
                            let string = format!("\"{}\"", key);
                            string
                        } else {
                            key.to_string()
                        }
                    } else {
                        key.to_string()
                    };
                    variants.push(SwitchVariant { name: prepare_key(key.as_str()), requirement, void: false, fields: None, single: Some(typ.get_path()) })
                }
            } else if let PacketDataType::Built { name: _, value } = value.as_ref() {
                generate_switch_native(parent_name, types, types_to_be_generated, &mut variants, &mut children, &key, value, &compare_to)?;
            } else if let PacketDataType::Native(native) = value.as_ref() {
                generate_switch_native(parent_name, types, types_to_be_generated, &mut variants, &mut children, &key, native, &compare_to)?;
            } else {
                warn!("Could not find type: {:?}", value);
            }
        }
    }
    debug!("Added {} to the types to be generated", parent_name);
    if let CompareTo::Specified { compare_to, compare_to_type } = compare_to {
        generates.push(GenerateType::SwitchEnum {
            content_name: parent_name.to_string(),
            wrap_in_mod: wrap_in_module,
            variants,
            children,
            compare_to,
            compare_to_type,
            generic_compare: false,
        });
    } else {
        generates.push(GenerateType::SwitchEnum {
            content_name: parent_name.to_string(),
            wrap_in_mod: wrap_in_module,
            variants,
            children,
            compare_to: String::new(),
            compare_to_type: String::new(),
            generic_compare: true,
        });
    }
    Ok(Some(generates).into_iter().flatten().collect())
}

fn generate_switch_native(parent_name: &TypeName, types: &mut Vec<PacketContentLocation>, types_to_be_generated: &PacketDataTypes, variants: &mut Vec<SwitchVariant>,
                          children: &mut Vec<GenerateType>, key: &&String, value: &NativeType, compare_to: &CompareTo) -> Result<(), GenError> {
    match value {
        NativeType::Container(values) => {
            let mut container_fields = Vec::with_capacity(values.len());
            let mut safe_string = false;
            for (name, value) in values {
                if let Some(typ) = find_type(value.as_ref(), types) {
                    let field = Field {
                        name: prepare_field(name),
                        data_type: typ.get_path(),
                        switch: typ.switch.clone(),
                    };
                    container_fields.push(field);
                } else {
                    if let PacketDataType::Native(native) = value.as_ref() {
                        if let NativeType::Switch { fields, default, compare_to } = native {
                            let value = container_fields
                                .iter()
                                .find(|field| field.name.eq(compare_to)).unwrap();
                            let type_name = TypeName::Named(format!("{}_Type", name).to_case(Case::UpperCamel));
                            let vec = create_switch(&type_name, false, types, fields, default, CompareTo::Specified {
                                compare_to: compare_to.to_string(),
                                compare_to_type: value.data_type.clone(),
                            }, types_to_be_generated).unwrap();
                            vec.into_iter().for_each(|typ| {
                                children.push(typ);
                            });
                            let field = Field {
                                name: prepare_field(name),
                                data_type: type_name.to_string(),
                                switch: Some(compare_to.to_string()),
                            };
                            container_fields.push(field);
                        }
                    }
                }
            }
            let requirement = if let CompareTo::Specified { compare_to_type, .. } = compare_to {
                if compare_to_type.eq("string") {
                    let string = format!("\"{}\"", key);
                    string
                } else {
                    key.to_string()
                }
            } else {
                key.to_string()
            };
            variants.push(SwitchVariant {
                requirement,
                name: prepare_key(key.as_str()),
                void: false,
                fields: Some(container_fields),
                single: None,
            });
        }
        NativeType::Switch { fields, default, compare_to } => {
            create_switch(&TypeName::Named(format!("{}_Type", key)), false, types, fields, default, CompareTo::Specified {
                compare_to: compare_to.to_string(),
                compare_to_type: "undefined".to_string(),
            }, types_to_be_generated)?.into_iter().for_each(|gen| {
                children.push(gen);
            });
        }
        ty => {
            warn!("Unsupported type: {:?}", ty);
        }
    }
    Ok(())
}