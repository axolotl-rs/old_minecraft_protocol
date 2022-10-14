pub mod packet_generator;
pub mod types;

use crate::{GenResult, Version};
use minecraft_data_rs::models::protocol::{PacketDataType, Protocol};

use std::collections::VecDeque;

use std::fs::{create_dir_all, OpenOptions};
use std::path::PathBuf;

use crate::code_gen::DataType;

use crate::error::GenError;
use crate::version_generator::protocol::packet_generator::PacketGenerator;
use convert_case::{Case, Casing};
use log::info;

use std::io::Write;

pub struct ProtocolGenerator {
    pub data_types: Vec<DataType>,
    pub types_to_be_generated: VecDeque<PacketDataType>,
    pub types_failed_to_generate: Vec<PacketDataType>,
}

pub fn generate_protocol(file: PathBuf, json: Protocol, _version: Version) -> GenResult<()> {
    let crate_path = format!("crate::protocol");
    create_dir_all(&file)?;

    info!("Generating protocol file: {}", file.display());
    let mut type_generator = types::TypesGenerator::new(json.types)?;
    let generates = type_generator.generate(&crate_path)?;
    let types_rs = file.join("types.rs");
    if !types_rs.exists() {
        let mut file_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&types_rs)?;
        for x in generates.into_iter() {
            let content = x.generate_type_wrap_as_mod().to_file_string().unwrap();
            file_file.write_all(content.as_bytes())?;
            file_file.write(b"\n")?;
        }
    }

    let mut packet_generator =
        packet_generator::PacketGenerator::new(&type_generator, json.handshaking)?;
    let mut handshake = file.join("handshake");
    create_packets(&mut packet_generator, &mut handshake);
    let mut packet_generator = packet_generator::PacketGenerator::new(&type_generator, json.login)?;
    let mut handshake = file.join("login");
    create_packets(&mut packet_generator, &mut handshake);
    let mut packet_generator =
        packet_generator::PacketGenerator::new(&type_generator, json.status)?;
    let mut handshake = file.join("status");
    create_packets(&mut packet_generator, &mut handshake);
    let mut packet_generator = packet_generator::PacketGenerator::new(&type_generator, json.play)?;
    let mut handshake = file.join("play");
    create_packets(&mut packet_generator, &mut handshake);
    Ok(()) // Everything is fine
}

fn create_packets(
    packet_generator: &mut PacketGenerator,
    handshake: &mut PathBuf,
) -> Result<(), GenError> {
    create_dir_all(&handshake)?;
    for generate in packet_generator.generate()?.into_iter() {
        let my_file = handshake.join(format!(
            "{}.rs",
            generate.content_name().to_case(Case::Snake)
        ));
        if my_file.exists() {
            continue;
        }
        let mut file_file = OpenOptions::new().write(true).create(true).open(&my_file)?;
        let content = generate.generate_with_imports().to_file_string().unwrap();
        file_file.write_all(content.as_bytes())?;
    }
    Ok(())
}
