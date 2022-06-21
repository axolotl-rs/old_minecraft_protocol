pub mod switch_compare;

use std::cmp::Ordering;
use crate::{GenResult};
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType, PacketDataTypes, Protocol};

use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::path::PathBuf;

use crate::error::GenError;
use crate::handlesbars::{Field, GenerateType};
use crate::version_generator::protocol::switch_compare::{create_switch, CompareTo};
use convert_case::{Case, Casing};
use log::{debug, error, info, warn};
use minecraft_data_rs::models::protocol::types::{TypeName};
use serde::{Deserialize};
use std::io::Write;
use std::process::Command;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct PacketContentLocation {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: PacketContentType,
    pub switch: Option<String>,

}

impl PacketContentLocation {
    pub fn create_field<'content>(&self, name: &str) -> Field {
        Field {
            name: name.to_string(),
            data_type: self.get_path(),
            switch: None,
        }
    }
    pub fn get_path(&self) -> String {
        match &self.typ {
            PacketContentType::Native { rust_type } => rust_type.clone(),

            PacketContentType::Manual => String::new(),
            PacketContentType::PreDefined { path } => path.clone(),

            PacketContentType::Generated { path } => path.clone(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum PacketContentType {
    Native {
        #[serde(rename = "type")]
        rust_type: String,
    },
    Manual,
    PreDefined {
        path: String,
    },
    Generated {
        path: String,
    },
}

pub fn generate_protocol(file: PathBuf, json: Protocol) -> GenResult<()> {
    create_dir_all(&file)?;

    info!("Generating protocol file: {}", file.display());
    generate_types(file.join("types.rs"), &json.types)?;

    Ok(()) // Everything is fine
}

pub fn generate_types(file: PathBuf, json: &PacketDataTypes) -> GenResult<()> {
    if file.exists() {
        warn!("File already exists");
        remove_file(&file)?;
    }
    let mut types: Vec<PacketContentLocation> = serde_json::from_slice(
        crate::Configs::get("default_type_impls.json")
            .unwrap()
            .data
            .as_ref(),
    )?;
    let mut types_to_generate = Vec::with_capacity(json.types.len());

    debug!("Generating the handlebars templates");
    let registry = crate::handlesbars::create_handlebars();
    for typ in &json.types {
        match typ {
            PacketDataType::Native(native) => {
                warn!("A new Native Type was added. {:?}", native);
            }
            PacketDataType::UnknownNativeType(unkown_native) => {
                warn!("A new Native Type was added. {:?}", unkown_native);
            }
            PacketDataType::Built { name, value } => {
                if types.iter().any(|typ| name.eq(&typ.name)) {
                    continue;
                }
                let option = generate_packet_content(name, value, true, &mut types, json)?;
                debug!("Adding {} types", option.len());
                option.into_iter().for_each(|x| {
                    types_to_generate.push(x);
                });
            }
            PacketDataType::Other(name, debug) => {
                warn!("A new type was added. {}. Debug Info {:?}. This will need to be fixed on the minecraft-data-rs level", name, debug);
            }
        }
    }
    let mut file_file = OpenOptions::new().write(true).create(true).open(&file)?;
    for x in types_to_generate.into_iter() {
        debug!("Generating {:?}", x);
        let content = x.generate(&registry).unwrap();
        file_file.write_all(content.as_bytes())?;
        file_file.write(b"\n")?;
    }
    drop(file_file);
    if let Err(error) = Command::new("rustfmt").arg(file).spawn() {
        warn!("Could not run rustfmt on the file: {}", error);
    }

    Ok(())
}

pub fn generate_packet_content(
    parent_name: &TypeName,
    parent_value: &NativeType,
    wrap_in_module: bool,
    types: &mut Vec<PacketContentLocation>,
    types_to_be_generated: &PacketDataTypes,
) -> Result<Vec<GenerateType>, crate::error::GenError> {
    let mut generates = Vec::with_capacity(1);

    match parent_value {
        NativeType::Container(inner_types) => {
            let mut container_fields = Vec::new();
            let mut children = Vec::new();
            for (_index, (typ_name, value)) in inner_types.iter().enumerate() {
                let option = find_type(value, types);

                if let Some(typ) = option {
                    container_fields.push(typ.create_field(&prepare_field(typ_name)));
                } else {
                    if !type_get(
                        parent_name,
                        parent_value,
                        wrap_in_module,
                        types,
                        types_to_be_generated,
                        &mut generates,
                        &mut container_fields,
                        &mut children,
                        typ_name,
                        value,
                    )? {
                        return Ok(generates);
                    }
                }
            }
            types.push(PacketContentLocation {
                typ: PacketContentType::Generated {
                    path: format!(
                        "super::{}::{}",
                        parent_name.to_string().to_case(Case::Snake),
                        parent_name.to_string().to_case(Case::UpperCamel)
                    ),
                },
                name: parent_name.to_string(),
                switch: None
            });
            generates.push(GenerateType::Full {
                content_name: parent_name.to_string(),
                fields: container_fields,
                wrap_in_mod: wrap_in_module,
                children,
            });
        }
        NativeType::Array {
            array_type,
            count_type,
        } => {
            let mut fields = Vec::new();

            let count_type = find_type_native(count_type.as_ref(), types).unwrap();
            fields.push(count_type.create_field("count"));
            if let Some(typ) = find_type(array_type.as_ref(), types) {
                let value = Field {
                    name: format!("{}_array", parent_name),
                    data_type: format!("Vec<{}>", typ.get_path()),
                    switch: None,
                };
                fields.push(value);
                generates.push(GenerateType::Full {
                    content_name: parent_name.to_string(),
                    fields,
                    wrap_in_mod: wrap_in_module,
                    children: vec![],
                });
            } else {
                match array_type.as_ref() {
                    PacketDataType::Built { name, value } => {
                        let children = generate_packet_content(
                            &TypeName::Named(format!("{}_array_type", name)),
                            value,
                            false,
                            types,
                            types_to_be_generated,
                        )?;
                        let value = Field {
                            name: format!("{}_array", name),
                            data_type: format!("Vec<{}>", format!("{}_array_type", name)),
                            switch: None,
                        };
                        fields.push(value);
                        generates.push(GenerateType::Full {
                            content_name: name.to_string(),
                            fields,
                            wrap_in_mod: wrap_in_module,
                            children,
                        });
                    }
                    _ => {
                        warn!("Could not find type: {:?}", array_type.as_ref());
                    }
                }
            }
            types.push(PacketContentLocation {
                typ: PacketContentType::Generated {
                    path: format!(
                        "super::{}::{}",
                        parent_name.to_string().to_case(Case::Snake),
                        parent_name.to_string().to_case(Case::UpperCamel)
                    ),
                },
                name: parent_name.to_string(),
                switch: None
            });
        }
        NativeType::Switch {
            fields,
            default,
            compare_to,
        } => {
            let compare_to = if compare_to.eq("$compareTo") {
                info!("Generating a generic switch");
                CompareTo::Generic
            } else {
                warn!("Can not create a Switch without context of a container");
                return Ok(vec![]);
            };
            let vec1 = create_switch(
                parent_name,
                true,
                types,
                fields,
                default,
                compare_to,
                types_to_be_generated,
            )?;
            info!(
                "Adding a switch with the type of {} adds {} extra generate values",
                parent_name,
                vec1.len()
            );
            vec1.into_iter().for_each(|x| {
                debug!("Adding a switch with the type of {:?}", x);
                generates.push(x);
            });
            types.push(PacketContentLocation {
                typ: PacketContentType::Generated {
                    path: format!(
                        "super::{}::{}",
                        parent_name.to_string().to_case(Case::Snake),
                        parent_name.to_string().to_case(Case::UpperCamel)
                    ),
                },
                name: parent_name.to_string(),
                switch: Some("$compareTo".to_string())
            });
        }
        NativeType::EntityMetadataLoop { .. } => {}
        v => {
            warn!("{} has an incompatible type of {:?}", parent_name, v);
        }
    }
    info!("Generated {} types", generates.len());
    Ok(generates)
}

fn type_get(
    parent_name: &TypeName,
    parent_value: &NativeType,
    wrap_in_module: bool,
    types: &mut Vec<PacketContentLocation>,
    types_to_be_generated: &PacketDataTypes,
    generates: &mut Vec<GenerateType>,
    container_fields: &mut Vec<Field>,
    children: &mut Vec<GenerateType>,
    typ_name: &TypeName,
    value: &Box<PacketDataType>,
) -> Result<bool, GenError> {
    match value.as_ref() {
        PacketDataType::Native(native) => {
            if let NativeType::Switch {
                fields,
                default,
                compare_to,
            } = native
            {
                let value = container_fields
                    .iter()
                    .find(|field| field.name.eq(compare_to));
                if let Some(field) = value {
                    let name1 = TypeName::Named(format!(
                        "{}Content",
                        parent_name.to_string().to_case(Case::UpperCamel)
                    ));
                    let result = switch_compare::create_switch(
                        &name1,
                        false,
                        types,
                        fields,
                        default,
                        CompareTo::Specified {
                            compare_to: compare_to.to_string(),
                            compare_to_type: field.data_type.clone(),
                        },
                        types_to_be_generated,
                    )?;
                    result.into_iter().for_each(|x| {
                        children.push(x);
                    });
                    container_fields.push(Field {
                        name: "content".to_string(),
                        data_type: name1.to_string(),
                        switch: Some(field.name.clone()),
                    })
                } else if value.is_none() {
                    warn!("Could not find the compare to field: {}", compare_to);
                }
            }
        }
        PacketDataType::Other(reference, _data) => {
                reference_lookup_create(
                    parent_name,
                    parent_value,
                    wrap_in_module,
                    types,
                    types_to_be_generated,
                    generates,
                    reference,
                )?;
            return Ok(false);
        }
        value => {
            warn!(
                "Could not build type for {} within {}",
                typ_name, parent_name
            );
            debug!("{:?}", value);
        }
    }
    Ok(true)
}

fn reference_lookup_create(
    parent_name: &TypeName,
    parent_value: &NativeType,
    wrap_in_module: bool,
    types: &mut Vec<PacketContentLocation>,
    types_to_be_generated: &PacketDataTypes,
    generates: &mut Vec<GenerateType>,
    reference: &String,
) -> Result<(), GenError> {
    debug!("Going to look for a reference for {}", reference);
    if let Some(value) = check_if_type_needs_to_be(reference, types_to_be_generated) {
        debug!("Found a reference for {} data {:?}", reference, value);

        if let PacketDataType::Built { name, value } = value {
            let vec2 = generate_packet_content(name, value, true, types, types_to_be_generated)?;
            if !vec2.is_empty() {
                vec2.into_iter().for_each(|x| {
                    generates.push(x);
                });
                info!("Attempting a regen on {}", parent_name);
                let vec3 = generate_packet_content(
                    parent_name,
                    parent_value,
                    wrap_in_module,
                    types,
                    types_to_be_generated,
                )?;
                vec3.into_iter().for_each(|x| {
                    generates.push(x);
                });
            } else {
                error!("Unable to build reference for {}", reference);
            }
        }
    }
    Ok(())
}

pub fn find_type<'types>(
    type_to: &PacketDataType,
    types: &'types Vec<PacketContentLocation>,
) -> Option<&'types PacketContentLocation> {
    for typ in types.iter() {
        match type_to {
            PacketDataType::Native(native) => {
                if typ.name.eq_ignore_ascii_case(native.get_name()) {
                    return Some(typ);
                }
            }

            PacketDataType::Other(hopefully_a_ref, _) => {
                if typ.name.eq_ignore_ascii_case(hopefully_a_ref) {
                    return Some(typ);
                }
            }
            _ => {}
        }
    }
    None
}

pub fn check_if_type_needs_to_be<'types>(
    hopefully_a_ref: &str,
    types: &'types PacketDataTypes,
) -> Option<&'types PacketDataType> {
    for typ in types.types.iter() {
        if let PacketDataType::Built { name, .. } = typ {
            if name.to_string().eq(hopefully_a_ref) {
                return Some(typ);
            }
        }
    }

    None
}

pub fn find_type_native<'types>(
    type_to: &NativeType,
    types: &'types Vec<PacketContentLocation>,
) -> Option<&'types PacketContentLocation> {
    for typ in types.iter() {
        if typ.name.eq(&type_to.get_name()) {
            return Some(typ);
        }
    }
    None
}

pub trait GetCall {
    fn generate_field(&self, name: &str) -> Option<Field>;
}

impl GetCall for NativeType {
    fn generate_field(&self, _name: &str) -> Option<Field> {
        None
    }
}

pub fn prepare_field(typ_: &TypeName) -> String {
    match typ_ {
        TypeName::Anonymous => "content".to_string(),
        TypeName::Named(name) => {
            if name.eq("type") {
                "data_type".to_string()
            } else if name.contains(":") {
                name.replace(":", "_")
            } else {
                name.to_string()
            }
        }
    }
}

pub fn prepare_key(str: &str) -> String {
    if str.eq("true") {
        "TrueSwitch".to_string()
    } else if str.eq("false") {
        "FalseSwitch".to_string()
    } else if i64::from_str(str).is_ok() {
        format!("Switch{}", str)
    } else if str.contains(":") {
        str.replace(":", "_")
    } else {
        str.to_string()
    }
}
