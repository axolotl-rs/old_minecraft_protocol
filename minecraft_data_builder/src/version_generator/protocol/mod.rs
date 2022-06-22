pub mod types;

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use crate::{GenResult};
use minecraft_data_rs::models::protocol::{NativeType, PacketDataType, PacketDataTypes, Protocol};

use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::path::PathBuf;

use crate::error::GenError;
use convert_case::{Case, Casing};
use log::{debug, error, info, warn};
use minecraft_data_rs::models::protocol::types::{TypeName};
use serde::{Deserialize};
use std::io::Write;
use std::process::Command;
use std::str::FromStr;
use crate::code_gen::DataType;
use crate::configs::type_impls::PacketContentType;

pub struct ProtocolGenerator {
    pub data_types: Vec<DataType>,
    pub types_to_be_generated: VecDeque<PacketDataType>,
    pub types_failed_to_generate: Vec<PacketDataType>,
}


pub fn generate_protocol(file: PathBuf, json: Protocol) -> GenResult<()> {
    create_dir_all(&file)?;

    info!("Generating protocol file: {}", file.display());
    let mut generator = types::TypesGenerator::new(json.types)?;
    let generates = generator.generate()?;
    let types_rs = file.join("types.rs");
    let mut file_file = OpenOptions::new().write(true).create(true).open(&types_rs)?;
    for x in generates.into_iter() {
        debug!("Generating {:?}", x);
        let content = x.generate_type().to_string().unwrap();
        println!("{}", content);
        file_file.write_all(content.as_bytes())?;
        file_file.write(b"\n")?;
    }

    Ok(()) // Everything is fine
}
